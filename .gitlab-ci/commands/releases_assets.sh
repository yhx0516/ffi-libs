#!/bin/bash
<<NOTES
    1. 解析从上个 stage 获取的 artifacts 内容
    2. 封装 assets 内容
NOTES

set -x

ls -l

# 从上个 stage 获取的 artifacts 内容
WINDOWS_ASSET_PATH=$CI_PROJECT_DIR/windows/1.txt
MAC_ASSET_PATH=$CI_PROJECT_DIR/mac/1.txt

TEMP_ASSET_PATH=$CI_PROJECT_DIR/temp.txt

if [ -e $WINDOWS_ASSET_PATH ]; then
    while IFS= read -r line
    do
        if [ ! -z $line ]; then
            echo "$line" >> $TEMP_ASSET_PATH
        fi
    done < "$WINDOWS_ASSET_PATH"
else
    echo "未生成 Windows 环境的 Asset 内容，请检查！"
fi

if [ -e $MAC_ASSET_PATH ]; then
    while IFS= read -r line
    do
        if [ ! -z $line ]; then
            echo "$line" >> $TEMP_ASSET_PATH
        fi
    done < "$MAC_ASSET_PATH"
else
    echo "未生成 Mac 环境的 Asset 内容，请检查！"
fi

if [ -e $TEMP_ASSET_PATH ]; then
    ROW_NUM=$(cat $TEMP_ASSET_PATH | wc -l)
    if [ $ROW_NUM -gt 1 ]; then
        ASSETS='{"links":['
        while IFS= read -r line
        do
            if [ ! -z $line ]; then
                if ((idx % 2 == 0)); then
                    if (( idx == 0 )); then
                        ASSETS="$ASSETS{\"url\":\"$line\","
                    else
                        ASSETS="$ASSETS,{\"url\":\"$line\","
                    fi
                else
                    ASSETS="$ASSETS\"name\":\"$line\"}"
                fi
                idx=$(( idx + 1 ))
            fi
        done < "$TEMP_ASSET_PATH"

        ASSETS="$ASSETS]}"

        echo "#########################"
        echo $ASSETS
        echo "#########################"

        source $CI_PROJECT_DIR/.gitlab-ci/commands/releases_update.sh "" "$ASSETS"
    else
       echo "$TEMP_ASSET_PATH 文件内容异常！"
       cat $TEMP_ASSET_PATH
    fi
fi