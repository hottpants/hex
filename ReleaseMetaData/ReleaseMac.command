#!/bin/bash
cd "$(dirname "$0")" || exit
/bin/bash ./GenerateIcon.command
/bin/bash ./Mac/MakeMacOSApp.command