#!/bin/bash
# exit on command fail
set -e

# Validates that user is root; exits if not
echo "Checking Root Privileges"
# shellcheck disable=SC2046
if [ $(id -u) -ne 0 ]
  then sudo bash "$0" "$@" && exit;
  else echo "User is root";
fi

# set dir correctly
cd "${0%/*}" || exit

# set the name of the Mac App
APP_NAME="Hex"
RUST_CRATE_NAME="Hex"
DMG_NAME="${APP_NAME}_release_mac.dmg"
PROJECT_DIR="../../" #relative to this files dir
RELEASE_FOLDER="${PROJECT_DIR}Release"
TMP_DMG_NAME="tmp.dmg"



# create the folder structure
mkdir -p "${RELEASE_FOLDER}/${APP_NAME}.app/Contents/MacOS"
mkdir -p "${RELEASE_FOLDER}/${APP_NAME}.app/Contents/Resources"

# copy Info.plist
cp Info.plist "${RELEASE_FOLDER}/${APP_NAME}.app/Contents/Info.plist"
# copy the icon (assuming you already have it in Apple ICNS format)
cp AppIcon.icns "${RELEASE_FOLDER}/${APP_NAME}.app/Contents/Resources/AppIcon.icns"
# copy your Bevy game assets
cp -a ../../assets "${RELEASE_FOLDER}/${APP_NAME}.app/Contents/MacOS/"

# compile the executables for each architecture
cd ${PROJECT_DIR} || exit
cargo build --release --target x86_64-apple-darwin # build for Intel
cargo build --release --target aarch64-apple-darwin # build for Apple Silicon

# combine the executables into a single file and put it in the bundle

lipo "target/x86_64-apple-darwin/release/${RUST_CRATE_NAME}" \
     "target/aarch64-apple-darwin/release/${RUST_CRATE_NAME}" \
     -create -output "Release/${APP_NAME}.app/Contents/MacOS/${APP_NAME}"


# cleanup old (just in case)
if [ -f "${TMP_DMG_NAME}" ]; then
    echo "Found old tmp DMG. Deleting..."
    rm "${TMP_DMG_NAME}" || { echo "Failed to delete old tmp DMG. Please check permissions and try again."; exit 1; }
    echo "Old tmp DMG deleted successfully."
fi

# Create a temporary DMG
hdiutil create -fs HFS+ -volname "${APP_NAME}" -srcfolder "Release/${APP_NAME}.app" -format UDRW "${TMP_DMG_NAME}"

# Mount the temporary DMG
echo "Mounting temporary DMG..."
MOUNT_OUTPUT=$(hdiutil attach -readwrite -noverify -noautoopen "${TMP_DMG_NAME}" 2>&1)

MOUNT_DIR=$(echo "${MOUNT_OUTPUT}" | grep 'Apple_HFS' | awk '{print $3}')
echo "Expected mount directory: '${MOUNT_DIR}'"

# wait for the mount point to be ready
MAX_ATTEMPTS=30
ATTEMPT=0
while [ ! -d "${MOUNT_DIR}" ] && [ $ATTEMPT -lt $MAX_ATTEMPTS ]; do
    echo "Waiting for mount point to be ready... (Attempt $((ATTEMPT+1))/$MAX_ATTEMPTS)"
    sleep 1
    ATTEMPT=$((ATTEMPT+1))
done

if [ ! -d "${MOUNT_DIR}" ]; then
    echo "Mount point '${MOUNT_DIR}' not ready after $MAX_ATTEMPTS attempts. Exiting."
     exit 1
fi

# Create a symlink to the Applications folder
ln -s /Applications "${MOUNT_DIR}/Applications"

# Optionally, set a custom background image
cp ReleaseMetaData/IconSource.png "${MOUNT_DIR}/.background.png"
SetFile -a V "${MOUNT_DIR}/.background.png" || echo "SetFile command failed, but continuing..."

# Clean up and unmount
echo "Attempting to unmount ${MOUNT_DIR}"
hdiutil detach "${MOUNT_DIR}" || {
    echo "Failed to unmount. Trying force unmount..."
    hdiutil detach "${MOUNT_DIR}" -force
}

# cleanup old
if [ -f "${DMG_NAME}" ]; then
    echo "Found old release DMG. Deleting..."
    rm "${DMG_NAME}" || { echo "Failed to delete old DMG. Please check permissions and try again."; exit 1; }
    echo "Old release DMG deleted successfully."
fi

# Convert the temporary DMG to the final compressed DMG
echo "Converting temporary DMG to final DMG"
hdiutil convert "${TMP_DMG_NAME}" -format UDZO -o "${DMG_NAME}" || {
    echo "Failed to convert DMG. Error: $?"
    exit 1
}

# Remove the temporary DMG
echo "Removing temporary DMG"
rm "${TMP_DMG_NAME}" || echo "Failed to remove temporary DMG, but continuing..."

echo "Script completed. Check if ${DMG_NAME} was created successfully."