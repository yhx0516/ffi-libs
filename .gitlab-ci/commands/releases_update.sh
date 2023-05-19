#!/bin/bash
<<NOTES
    对 GitLab 的 DeployMenets -> Release 进行增删改
    https://docs.gitlab.com/ee/api/releases/
    该脚本仅在 Mac 环境验证
NOTES

RELEASE_URL=$CI_SERVER_URL/api/v4/projects/$CI_PROJECT_ID/releases # "https://git.sofunny.io/api/v4/projects/1148/releases"
DESCRIPTION=$1 # description
ASSETS=$2 # assets {"links":[{"url":"https://example.com/assets/1","name":"asset1"}]}

if [ ! -z "$DESCRIPTION" ] && [ ! -z "$ASSETS" ]; then
    DATA="{\"name\": \"$CI_COMMIT_TAG\", \"tag_name\": \"$CI_COMMIT_TAG\", \"description\": \"${DESCRIPTION}\", \"assets\": \"${ASSETS}\" }"
elif [ ! -z "$DESCRIPTION" ]; then
    DATA="{\"name\": \"$CI_COMMIT_TAG\", \"tag_name\": \"$CI_COMMIT_TAG\", \"description\": \"${DESCRIPTION}\" }"
elif [ ! -z "$ASSETS" ]; then
    DATA="{\"name\": \"$CI_COMMIT_TAG\", \"tag_name\": \"$CI_COMMIT_TAG\", \"assets\": ${ASSETS} }"
else
    DATA="{\"name\": \"$CI_COMMIT_TAG\", \"tag_name\": \"$CI_COMMIT_TAG\" }"
fi

# Get Release From Projects
RESPONESE=`curl --request GET --header "PRIVATE-TOKEN: $GITLAB_TOKEN" $RELEASE_URL/$CI_COMMIT_TAG`
IS_EXIST=`echo "$RESPONESE" | grep "\"tag_name\":\"$CI_COMMIT_TAG\"" | wc  -l`
if [ $IS_EXIST -gt 0 ]; then # Update Release Info
    echo "Release $CI_COMMIT_TAG Exists."
    RESPONESE=`curl --request PUT --header "PRIVATE-TOKEN: $GITLAB_TOKEN" --header "Content-Type: application/json" \
        --data "$DATA" \
        $RELEASE_URL/$CI_COMMIT_TAG`
    IS_EXIST=`echo "$RESPONESE" | grep "\"tag_name\":\"$CI_COMMIT_TAG\"" | wc  -l`
    if [ ! $IS_EXIST -gt 0 ]; then
        echo "Update Release Failed."
    fi
else # Create Release
    echo "Create Release $CI_COMMIT_TAG."
    RESPONESE=`curl --request POST --header "PRIVATE-TOKEN: $GITLAB_TOKEN" --header "Content-Type: application/json" \
        --data "$DATA" \
        $RELEASE_URL`
    IS_EXIST=`echo "$RESPONESE" | grep "\"tag_name\":\"$CI_COMMIT_TAG\"" | wc  -l`
    if [ ! $IS_EXIST -gt 0 ]; then
        echo "Create Release Failed."
    fi
fi