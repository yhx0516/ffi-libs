#!/bin/bash
<<NOTES
    1. Tag 触发，构建指定平台的 rust 库
    2. 上传到 ftp，生成地址信息上传至 artifacts 供后续 Job 获取

    Notes: 
        1. 需安装 Rust 与 Cross
        2. 构建 Android 需安装 Docker
NOTES

set -x
ls -l

which cargo
cargo --version

BUILD_TAG=$CI_COMMIT_TAG
if [ -z $BUILD_TAG ]; then
    BUILD_TAG="0.0.0"
fi

# 模式匹配，需用 [[]]
# example:
#   BUILD_TAG = rpkg-0.2.9
#   SUFFIX = 0.2.9
if [[ $BUILD_TAG =~ -(.+)$ ]]; then
    SUFFIX=${BASH_REMATCH[1]}
    BUILD_TAG=$SUFFIX
fi

# example:
#   CI_COMMIT_TAG = rpkg-0.2.9
#   PREFIX        = rpkg
if [[ $CI_COMMIT_TAG =~ ^(.+)- ]]; then
    PREFIX=${BASH_REMATCH[1]}
    PACKAGE_ARR=($PREFIX)
else
    echo "无法从 TAG: $CI_COMMIT_TAG 中提取 - 前面的内容"
    # PACKAGE_ARR=(rtoml rpkg rhandlebars)
    IFS=' ' read -ra PACKAGE_ARR <<< "$BUILD_PACKAGE"
fi

if [ $OSTYPE == "msys" ]; then
    echo "Windows"
    TARGET_ARR=(x86_64-pc-windows-msvc aarch64-linux-android)
else
    echo "MacOS"
    TARGET_ARR=(aarch64-apple-darwin)
fi

BUILE_TYPE=release

# 保证 FTP 文件夹存在
RELEASE_ROOT=GitLab
RELEASE_DIR=$RELEASE_ROOT/$CI_PROJECT_NAME/$BUILD_TAG/$BUILE_TYPE
mkdir -p $RELEASE_DIR
/usr/bin/scp -B -r $RELEASE_ROOT lubin@10.20.3.42:/data/research
rm -rf $RELEASE_ROOT

mkdir $VAR_ASSET_DIR
for CUR_PACKAGE in ${PACKAGE_ARR[@]}; do
    echo "当前构建 Package: $CUR_PACKAGE"
    
    for CUR_TARGET in ${TARGET_ARR[@]}; do
        echo "当前构建平台: $CUR_TARGET"

        cd $CI_PROJECT_DIR
        OUTPUT_DIR=target/$CUR_TARGET/$BUILE_TYPE
        rm -rf $OUTPUT_DIR

        # 统一构建格式，实际只要 android 需要 cross
        cross build --target $CUR_TARGET -p $CUR_PACKAGE --$BUILE_TYPE
        if [ -e $OUTPUT_DIR ]; then
            cd $OUTPUT_DIR

            IS_EXIST=1
            case $CUR_TARGET in
                "x86_64-pc-windows-msvc") # windows -> .dll
                    echo "x86_64-pc-windows-msvc."
                    PACKAGE_NAME=`ls | grep -Eo ".+.dll$" | tail -1`
                    if  [ -z $PACKAGE_NAME ]; then
                        echo "构建 $CUR_TARGET 时，未在 $OUTPUT_DIR 找到任何 dll！"
                    fi
                ;;
                "aarch64-linux-android") # android -> .so
                    echo "aarch64-linux-android."
                    PACKAGE_NAME=`ls | grep -Eo ".+.so$" | tail -1`
                    if  [ -z $PACKAGE_NAME ]; then
                        echo "构建 $CUR_TARGET 时，未在 $OUTPUT_DIR 找到任何 so！"
                    fi
                ;;
                "aarch64-apple-darwin") # mac -> dylib
                    echo "aarch64-apple-darwin."
                    PACKAGE_NAME=`ls | grep -Eo ".+.dylib$" | tail -1`
                    if  [ -z $PACKAGE_NAME ]; then
                        echo "构建 $CUR_TARGET 时，未在 $OUTPUT_DIR 找到任何 dylib！"
                    fi
                ;;
                *)
                    IS_EXIST=0
                    echo "unknown target: $CUR_TARGET！"
                ;;
            esac

            # 压缩上传
            if [ $IS_EXIST -eq 1 ]; then
                TAR_NAME=$CUR_PACKAGE-$BUILD_TAG-$CUR_TARGET.tar
                
                # 方式1 只上传 dll so dylib
                # tar -zcf $TAR_NAME $PACKAGE_NAME

                # 方式2 上传目录下除文件夹外的所有
                shopt -s dotglob # 启用 dotglob
                for file in *
                do
                    if [ -d $file ]; then # 删除文件夹
                        rm -rf $file
                    fi
                done
                shopt -u dotglob # 禁用 dotglob
                tar -zcf $TAR_NAME .

                /usr/bin/scp -B $TAR_NAME lubin@10.20.3.42:/data/research/$RELEASE_DIR/$TAR_NAME
                rm -rf $TAR_NAME

                # 写入 Assets
                echo "https://dl.sofunny.io/$RELEASE_DIR/$TAR_NAME" >> $VAR_ASSET_PATH
                echo "$TAR_NAME" >> $VAR_ASSET_PATH
                cat $VAR_ASSET_PATH
            fi
        else
            echo "构建异常，未生成任何库文件！"
        fi
    done
done