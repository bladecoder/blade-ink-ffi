#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <stdbool.h>
#include <bink.h>


void finish(int rc, struct bink_story *story, char *err_msg) {
    bink_cstring_free(err_msg);
    bink_story_free(story);
    exit(rc);
}

void check_ret(int ret, struct bink_story *story, char *err_msg) {
    if (ret != BINK_OK) {
        if(err_msg != NULL)
            perror(err_msg);
        
        finish(EXIT_FAILURE, story, err_msg);
    }
}

void print_choices(struct bink_choices *choices, size_t len) {
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


int main(void) {
    uint32_t ret = BINK_OK;
    struct bink_story *story = NULL;
    struct bink_choices *choices = NULL;
    char *err_msg = NULL;
    char *line = NULL;
    // char *json_string = "{\"inkVersion\":21,\"root\":[[\"^Line.\",\"\\n\",[\"done\",{\"#n\":\"g-0\"}],null],\"done\",null],\"listDefs\":{}}";

    char *json_string = read_json_file("./inkfiles/TheIntercept.ink.json");
    if(json_string == NULL)
        exit(EXIT_FAILURE);

    ret = bink_story_new(&story, json_string, &err_msg);
    check_ret(ret, story, err_msg);
    free(json_string);

    bool end = false;

    while(!end) {
        bool can_continue;
        ret = bink_story_can_continue(story, &can_continue);
        check_ret(ret, story, err_msg);

        while (can_continue) {
            ret = bink_story_cont(story, &line, &err_msg);
            check_ret(ret, story, err_msg);
            puts(line);
            bink_cstring_free(line);
            
            ret = bink_story_can_continue(story, &can_continue);
            check_ret(ret, story, err_msg);
        }

        // Obtain and print choices
        size_t len = 0;
        ret = bink_story_get_current_choices(story, &choices, &len);
        check_ret(ret, story, NULL);
        //printf("Num. choices: %lu\n", len);

        if (len !=0) {
            print_choices(choices, len);
            printf("\n");
            bink_choices_free(choices);

            // Always choose the first option
            ret = bink_story_choose_choice_index(story, 0);
            check_ret(ret, story, NULL);
        } else {
            end = true;
        }

    }

    printf("Story ended ok.\n");

    finish(EXIT_SUCCESS, story, err_msg);
}