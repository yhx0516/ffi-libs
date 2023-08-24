#!/bin/bash
<<NOTES
    1. Tag 触发
        1.1 构建指定平台的 rust 库，上传到 ftp
        1.2 生成地址信息上传至 artifacts 供后续 Job 获取
        1.3 自动更新 Deployments/Releases
    2. 手动触发
        2.1 构建指定 $BUILD_PACKAGE 的库，上传到 ftp
    Notes: 
        1. 需安装 Rust 与 Cross
        2. 构建 Android 需安装 Docker
        3. 当前 Rust 版本为 1.70.0
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

# 手动模式，BUILD_TAG 为当前分支
if [ $CI_PIPELINE_SOURCE == "web" ] || [ $CI_PIPELINE_SOURCE == "schedule" ]; then
    ehco $CI_COMMIT_BRANCH
    # 将分支的 \ 替换成 -
    BUILD_TAG=`echo $CI_COMMIT_BRANCH | sed 's/\//-/g'`
fi 

if [ $OSTYPE == "msys" ]; then
    echo "Windows"
    TARGET_ARR=(x86_64-pc-windows-msvc aarch64-linux-android x86_64-unknown-linux-gnu)
else
    echo "MacOS"
    TARGET_ARR=(aarch64-apple-ios aarch64-apple-darwin x86_64-apple-darwin)
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

    local aarch64_darwin_name=""
    local x86_64_darwin_name=""
    
    for CUR_TARGET in ${TARGET_ARR[@]}; do
        echo "当前构建平台: $CUR_TARGET"

        cd $CI_PROJECT_DIR
        OUTPUT_DIR=target/$CUR_TARGET/$BUILE_TYPE
        rm -rf $OUTPUT_DIR

        # 统一构建格式，实际只有 android、linux 需要 cross
        cross build --target $CUR_TARGET -p $CUR_PACKAGE --$BUILE_TYPE
        if [ -e $OUTPUT_DIR ]; then
            cd $OUTPUT_DIR

            IS_EXIST=1
            echo "$CUR_TARGET"
            case $CUR_TARGET in
                "x86_64-pc-windows-msvc") # windows -> .dll
                    PACKAGE_NAME=`ls | grep -Eo ".+.dll$" | tail -1`
                    if  [ -z $PACKAGE_NAME ]; then
                        IS_EXIST=0
                        echo "构建 $CUR_TARGET 时，未在 $OUTPUT_DIR 找到任何 dll！"
                    fi
                ;;
                "aarch64-linux-android" | "x86_64-unknown-linux-gnu") # android linux -> .so
                    PACKAGE_NAME=`ls | grep -Eo ".+.so$" | tail -1`
                    if  [ -z $PACKAGE_NAME ]; then
                        IS_EXIST=0
                        echo "构建 $CUR_TARGET 时，未在 $OUTPUT_DIR 找到任何 so！"
                    fi
                ;;
                "aarch64-apple-ios") # ios -> .a
                    PACKAGE_NAME=`ls | grep -Eo ".+.a$" | tail -1`
                    if  [ -z $PACKAGE_NAME ]; then
                        IS_EXIST=0
                        echo "构建 $CUR_TARGET 时，未在 $OUTPUT_DIR 找到任何 a！"
                    fi
                ;;
                "aarch64-apple-darwin") # mac apple -> .dylib
                    PACKAGE_NAME=`ls | grep -Eo ".+.dylib$" | tail -1`
                    if  [ -z $PACKAGE_NAME ]; then
                        IS_EXIST=0
                        echo "构建 $CUR_TARGET 时，未在 $OUTPUT_DIR 找到任何 dylib！"
                    else
                        aarch64_darwin_name=$PACKAGE_NAME
                    fi
                ;;
                "x86_64-apple-darwin") # mac intel -> .dylib
                    PACKAGE_NAME=`ls | grep -Eo ".+.dylib$" | tail -1`
                    if  [ -z $PACKAGE_NAME ]; then
                        IS_EXIST=0
                        echo "构建 $CUR_TARGET 时，未在 $OUTPUT_DIR 找到任何 dylib！"
                    else
                        x86_64_darwin_name=$PACKAGE_NAME
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
                # tar -cf $TAR_NAME $PACKAGE_NAME

                # 方式2 上传目录下除文件夹外的所有
                shopt -s dotglob # 启用 dotglob
                for file in *
                do
                    if [ -d $file ]; then # 删除文件夹
                        rm -rf $file
                    fi
                done
                shopt -u dotglob # 禁用 dotglob
                tar -cf $TAR_NAME .

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

    # 将 aarch64-apple-darwin 与 x86_64-apple-darwin 合成为 universal-apple-darwin
    if [ ! -z $aarch64_darwin_name ] && [ ! -z $x86_64_darwin_name ]; then
        echo "开始合包"
        cd $CI_PROJECT_DIR
        UNIVERSAL_DIR=target/universal-apple-darwin
        rm -rf $UNIVERSAL_DIR
        mkdir $UNIVERSAL_DIR
        lipo -create -output $UNIVERSAL_DIR/$aarch64_darwin_name target/aarch64-apple-darwin/release/$aarch64_darwin_name target/x86_64-apple-darwin/release/$x86_64_darwin_name
    
        # 压缩上传
        cd $UNIVERSAL_DIR
        TAR_NAME=$CUR_PACKAGE-$BUILD_TAG-universal-apple-darwin.tar
        tar -cf $TAR_NAME .
        /usr/bin/scp -B $TAR_NAME lubin@10.20.3.42:/data/research/$RELEASE_DIR/$TAR_NAME
        rm -rf $TAR_NAME

        # 写入 Assets
        echo "https://dl.sofunny.io/$RELEASE_DIR/$TAR_NAME" >> $VAR_ASSET_PATH
        echo "$TAR_NAME" >> $VAR_ASSET_PATH
        cat $VAR_ASSET_PATH
    fi
done