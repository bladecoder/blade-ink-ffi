#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <stdbool.h>
#include <string.h>
#include <bink.h>

void finish(int rc, BINK_Story *story, char *err_msg) {
    bink_cstring_free(err_msg);
    bink_story_free(story);
    exit(rc);
}

void check_ret(int ret, BINK_Story *story, char *err_msg) {
    if (ret != BINK_OK) {
        if(err_msg != NULL)
            perror(err_msg);
        
        finish(EXIT_FAILURE, story, err_msg);
    }
}

void print_choices(BINK_Choices *choices, size_t len) {
    for (size_t i=0; i < len; i++) {
        char *text = NULL;
        int ret = bink_choices_get_text(choices, i, &text);
        if (ret != BINK_OK) {
            finish(EXIT_FAILURE, NULL, NULL);
        }

        printf("%lu. %s\n", i+1, text);
        bink_cstring_free(text);
    }
}

char* read_json_file(const char* filename) {
    FILE* file = fopen(filename, "r");
    if (!file) {
        perror("Failed to open file");
        return NULL;
    }

    fseek(file, 0, SEEK_END);
    long fileSize = ftell(file);
    fseek(file, 0, SEEK_SET);

    char* jsonString = (char*)malloc(fileSize + 1);
    if (!jsonString) {
        perror("Memory allocation failed");
        fclose(file);
        return NULL;
    }

    size_t bytesRead = fread(jsonString, 1, fileSize, file);
    if ((long)bytesRead != fileSize) {
        perror("Failed to read file");
        free(jsonString);
        fclose(file);
        return NULL;
    }

    jsonString[fileSize] = '\0';

    fclose(file);

    return jsonString;
}

void assert_cont(BINK_Story *story, char *expected) {
    char *err_msg = NULL;
    char *line = NULL;

    int ret = bink_story_cont(story, &line, &err_msg);
    check_ret(ret, story, err_msg);

    if (strcmp(line, expected) != 0) {
        printf("expected: %s, found: %s\n", expected, line);
        finish(EXIT_FAILURE, NULL, NULL);
    }

    bink_cstring_free(line);
}
