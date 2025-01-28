VER=0.1.0
DIST_DIR=dist
PROGRAM=aes256gen

function build() {
    target=$1
    tool=$2
    profile=$3
    exe=$4
    target_dir=./target/$target/$profile/
    artefact=$PROGRAM-$VER-$target.tar.gz
    rm -rf $target_dir
    $tool build --target=$target --profile $profile
    (cd $target_dir && tar czvf $artefact $exe && cp $artefact ../../../$DIST_DIR/)
}

rm -rf ./$DIST_DIR/
mkdir ./$DIST_DIR/
build x86_64-unknown-linux-musl cargo release $PROGRAM
build x86_64-pc-windows-msvc cross release $PROGRAM.exe