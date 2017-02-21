#!/bin/sh

#
# This is a simple configure script which generates files for a freedesktop.org 
# compatible desktop environment.
#

# Defaults for a local install
PREFIX="."
BIN_PREFIX="./target/release"
SHARE_PREFIX=`pwd`
DATA_PREFIX=`pwd`

# Parse command-line arguments
while [[ $# -gt 0 ]]
do
    key="$1"

    case $key in
        --prefix)
            # Installation prefix
            PREFIX="$2"
            if [ "$PREFIX" != "." ] ; then
                BIN_PREFIX="$2/bin"
                SHARE_PREFIX="$2/share"
                DATA_PREFIX="$2/share/garta"
            fi
            shift
            ;;
        *)
            echo "./configure.sh [options]"
            echo ""
            echo "Available option"
            echo ""
            echo "    -prefix <dir> ... The deployment directory, default /usr/local"
            echo ""
            exit 1
            ;;
    esac
    shift
done

# Substitute PREFIX
CONFIG_FILENAME=src/core/_config.rs
INSTALL_FILENAME=install.sh
UNINSTALL_FILENAME=uninstall.sh
DESKTOP_FILENAME=com.github.zaari.garta.desktop
VERSION=`grep version Cargo.toml | cut -d '=' -f 2 | sed 's/[ \"]//g'`

# -------------------------------------------------------------------------------------------------
cat > $CONFIG_FILENAME << EOF
/// -------------------------------------------------------------------------
/// This file was generated by configure.sh. Do not add to version control.
/// -------------------------------------------------------------------------

pub static DATA_PREFIX: &'static str = "$DATA_PREFIX";

EOF
echo "Created file $CONFIG_FILENAME"

# -------------------------------------------------------------------------------------------------
if [ "$PREFIX" != "." ] ; then
cat > $INSTALL_FILENAME << EOF
#!/bin/bash
set -e

if [ ! -d "$BIN_PREFIX" ] ; then 
    mkdir -p $BIN_PREFIX
fi

cp -p target/release/garta $BIN_PREFIX/
chown $USER:$GROUP $BIN_PREFIX/garta

if [ ! -d "$SHARE_PREFIX/applications/" ] ; then 
    mkdir -p $SHARE_PREFIX/applications/
fi
cp -p $DESKTOP_FILENAME $SHARE_PREFIX/applications/
chown $USER:$GROUP $SHARE_PREFIX/applications/$DESKTOP_FILENAME

if [ ! -d "$SHARE_PREFIX/icons/hicolor/scalable/apps/" ] ; then 
    mkdir -p $SHARE_PREFIX/icons/hicolor/scalable/apps/
fi
cp -p icons/garta.svg $PREFIX/share/icons/hicolor/scalable/apps/
chown $USER:$GROUP $PREFIX/share/icons/hicolor/scalable/apps/garta.svg

if [ ! -d "$DATA_PREFIX" ] ; then 
    mkdir -p $DATA_PREFIX
    mkdir -p $DATA_PREFIX/ui
    mkdir -p $DATA_PREFIX/maps
fi
cp -pR maps/* $DATA_PREFIX/maps/
chown -R $USER:$GROUP $DATA_PREFIX/maps/*
cp -pR ui/* $DATA_PREFIX/ui/
chown -R $USER:$GROUP $DATA_PREFIX/ui/*
EOF
else
cat > $INSTALL_FILENAME << EOF
#!/bin/bash
set -e

cp -p $DESKTOP_FILENAME $HOME/.local/share/applications/
EOF
fi
chmod u+x $INSTALL_FILENAME
echo "Created file $INSTALL_FILENAME"


# -------------------------------------------------------------------------------------------------
if [ "$PREFIX" != "." ] ; then
cat > $UNINSTALL_FILENAME << EOF
#!/bin/bash

rm -f $BIN_PREFIX/garta
rm -f $SHARE_PREFIX/applications/com.github.zaari.garta.desktop
rm -f $SHARE_PREFIX/icons/hicolor/scalable/apps/garta.svg
rm -fR $DATA_PREFIX

EOF
else
cat > $UNINSTALL_FILENAME << EOF
#!/bin/bash

rm -fR $HOME/.local/share/applications/$DESKTOP_FILENAME
EOF
fi
chmod u+x $UNINSTALL_FILENAME
echo "Created file $UNINSTALL_FILENAME"

# -------------------------------------------------------------------------------------------------
if [ "$PREFIX" != "." ] ; then
    ICON_FILE=$SHARE_PREFIX/icons/hicolor/scalable/apps/garta.svg
    EXECUTABLE=$BIN_PREFIX/garta
else
    ICON_FILE=`pwd`/icons/garta.svg
    EXECUTABLE=`pwd`/target/release/garta
fi
cat > $DESKTOP_FILENAME << EOF
[Desktop Entry]
Type=Application
Categories=GTK;Education;Geography
Keywords=Maps;GPX;Geography;Track;Route;
#MimeType=application/gpx+xml;application/xml
Encoding=UTF-8
Name=Garta
Version=$VERSION
Comment=Garta GPX viewer
Exec=$EXECUTABLE
Icon=$ICON_FILE
Terminal=false
EOF
echo "Created file $DESKTOP_FILENAME"

