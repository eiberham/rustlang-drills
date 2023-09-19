#!/bin/bash

set -e

APP_NAME=tetris
MACOS_BIN_NAME=tetris-bin
MACOS_APP_NAME=Tetris
MACOS_APP_DIR=$MACOS_APP_NAME.app

mkdir -p macos
cd macos
echo "Creating app directory structure"
rm -rf $MACOS_APP_NAME
rm -rf $MACOS_APP_DIR
mkdir -p $MACOS_APP_DIR/Contents/MacOS

cargo rustc \
    --verbose \
    --release

echo "Copying binary"
MACOS_APP_BIN=$MACOS_APP_DIR/Contents/MacOS/$MACOS_BIN_NAME
cp ../target/release/$APP_NAME $MACOS_APP_BIN

echo "Copying resources directory"
cp -r ../resources $MACOS_APP_DIR/Contents/MacOS

echo "Copying launcher"
cp ../exec.sh $MACOS_APP_DIR/Contents/MacOS/$MACOS_APP_NAME

echo "Copying Icon"
mkdir -p $MACOS_APP_DIR/Contents/Resources
mv ../resources/Info.plist $MACOS_APP_DIR/Contents/
mv ../resources/snake.icns $MACOS_APP_DIR/Contents/Resources/

echo "Creating dmg"
mkdir -p $MACOS_APP_NAME
cp -r $MACOS_APP_DIR $MACOS_APP_NAME/
rm -rf $MACOS_APP_NAME/.Trashes

FULL_NAME=$MACOS_APP_NAME

hdiutil create $FULL_NAME.dmg -srcfolder $MACOS_APP_NAME -ov
rm -rf $MACOS_APP_NAME