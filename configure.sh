#!/bin/bash

#
# This is a simple configure script which generates files for a freedesktop.org 
# compatible desktop environment.
#

# Defaults for a local install
PREFIX="."
EXEC_PREFIX="./target/release"
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
                EXEC_PREFIX="$2/bin"
                SHARE_PREFIX="$2/share"
                DATA_PREFIX="$2/share/garta"
            fi
            shift
            ;;
        --exec-prefix)
            EXEC_PREFIX="$PREFIX/bin"
            shift
            ;;
        *)
            echo "./configure.sh [options]"
            echo ""
            echo "Available options:"
            echo "    --prefix <dir>       The deployment directory [/usr/local]"
            echo "    --exec-prefix <dir>  The deployment directory [$PREFIX/bin]"
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
FIREJAIL_FILENAME=garta.profile
APP_VERSION=`grep version Cargo.toml | cut -d '=' -f 2 | sed 's/[ \"]//g'`

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

umask 0022

if [ "\$DESTDIR" != ""  ] ; then
    if [ ! -d "\$DESTDIR" ] ; then 
        echo "\$DESTDIR must exist" &>/dev/stderr
        exit 1
    fi
    DESTDIR="\$DESTDIR/"
fi

if [ ! -d "\$DESTDIR$EXEC_PREFIX" ] ; then 
    mkdir -p \$DESTDIR$EXEC_PREFIX
fi

cp -p target/release/garta \$DESTDIR$EXEC_PREFIX/
chown $USER:$GROUP \$DESTDIR$EXEC_PREFIX/garta

if [ ! -d "\$DESTDIR$SHARE_PREFIX/applications/" ] ; then 
    mkdir -p \$DESTDIR$SHARE_PREFIX/applications/
fi
cp -p $DESKTOP_FILENAME \$DESTDIR$SHARE_PREFIX/applications/
chown $USER:$GROUP \$DESTDIR$SHARE_PREFIX/applications/$DESKTOP_FILENAME

if [ ! -d "\$DESTDIR$SHARE_PREFIX/icons/hicolor/scalable/apps/" ] ; then 
    mkdir -p \$DESTDIR$SHARE_PREFIX/icons/hicolor/scalable/apps/
fi
cp -p icons/garta.svg \$DESTDIR$PREFIX/share/icons/hicolor/scalable/apps/
chown $USER:$GROUP \$DESTDIR$PREFIX/share/icons/hicolor/scalable/apps/garta.svg

if [ ! -d "\$DESTDIR/etc/firejail/" ] ; then 
    mkdir -p \$DESTDIR/etc/firejail/
fi
cp -p garta.profile \$DESTDIR/etc/firejail/
chown $USER:$GROUP \$DESTDIR/etc/firejail/garta.profile

if [ ! -d "\$DESTDIR$DATA_PREFIX" ] ; then 
    mkdir -p \$DESTDIR$DATA_PREFIX
    mkdir -p \$DESTDIR$DATA_PREFIX/ui
    mkdir -p \$DESTDIR$DATA_PREFIX/maps
fi
cp -pR maps/* \$DESTDIR$DATA_PREFIX/maps/
chown -R $USER:$GROUP \$DESTDIR$DATA_PREFIX/maps/*
cp -pR ui/* \$DESTDIR$DATA_PREFIX/ui/
chown -R $USER:$GROUP \$DESTDIR$DATA_PREFIX/ui/*
EOF
else
cat > $INSTALL_FILENAME << EOF
#!/bin/bash
set -e

if [ ! -d "$HOME/.local/share/applications" ] ; then 
    mkdir -p $HOME/.local/share/applications
fi
cp -p $DESKTOP_FILENAME $HOME/.local/share/applications/
EOF
fi
chmod u+x $INSTALL_FILENAME
echo "Created file $INSTALL_FILENAME"


# -------------------------------------------------------------------------------------------------
if [ "$PREFIX" != "." ] ; then
cat > $UNINSTALL_FILENAME << EOF
#!/bin/bash

if [ "\$DESTDIR" != ""  ] ; then
    DESTDIR="\$DESTDIR/"
fi

rm -f \$DESTDIR$EXEC_PREFIX/garta
rm -f \$DESTDIR$SHARE_PREFIX/applications/com.github.zaari.garta.desktop
rm -f \$DESTDIR$SHARE_PREFIX/icons/hicolor/scalable/apps/garta.svg
rm -fR \$DESTDIR$DATA_PREFIX
rm -f \$DESTDIR/etc/firejail/garta.profile

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
if [ -f /usr/bin/firejail ] && [ "$PREFIX" != "." ] ; then
    FIREJAIL_COMMAND=/usr/bin/firejail
fi
if [ "$PREFIX" != "." ] ; then
    ICON_FILE=$SHARE_PREFIX/icons/hicolor/scalable/apps/garta.svg
    EXECUTABLE=$EXEC_PREFIX/garta
else
    ICON_FILE=`pwd`/icons/garta.svg
    EXECUTABLE=`pwd`/target/release/garta
fi
cat > $DESKTOP_FILENAME << EOF
[Desktop Entry]
Type=Application
Categories=GTK;Education;Geography
Keywords=map;geography;track;GPX;GPS;
Encoding=UTF-8
Name=Garta
Version=$APP_VERSION
Comment=Garta GPX viewer
TryExec=$EXECUTABLE
Exec=$FIREJAIL_COMMAND $EXECUTABLE
Icon=$ICON_FILE
Terminal=false
EOF
#Keywords=map;geography;track;route;waypoint;GPX;GPS;
#MimeType=application/gpx+xml;application/xml
echo "Created file $DESKTOP_FILENAME"

# -------------------------------------------------------------------------------------------------
cat > $FIREJAIL_FILENAME << EOF
# Firejail profile for Garta application

include /etc/firejail/garta.local
include /etc/firejail/disable-common.inc
include /etc/firejail/disable-devel.inc
include /etc/firejail/disable-passwdmgr.inc
include /etc/firejail/disable-programs.inc

read-only ~/.config/gtk-3.0
whitelist ~/.config/gtk-3.0

mkdir ~/.config/garta
whitelist ~/.config/garta
mkdir ~/.local/share/garta
whitelist ~/.local/share/garta
mkdir ~/.cache/garta
whitelist ~/.cache/garta

name garta
nosound
caps.drop all
netfilter
nonewprivs
noroot
nogroups
protocol unix,inet,inet6
seccomp
shell none
tracelog

private-dev
private-tmp
EOF
echo "Created file $FIREJAIL_FILENAME"

