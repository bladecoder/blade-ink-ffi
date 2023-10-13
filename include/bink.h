#ifndef _LIBBINK_H_
#define _LIBBINK_H_

#ifdef __cplusplus
extern "C" {
#endif

#include <stdint.h>
#include <stdbool.h>

#define BINK_OK                   0
#define BINK_FAIL                 1
#define BINK_FAIL_NULL_POINTER    2

typedef void BINK_Story;
typedef void BINK_Choices;
typedef void BINK_Tags;

int bink_story_new(BINK_Story **story, char *json_string, char **err_msg);
void bink_story_free(BINK_Story *story);
int bink_story_can_continue(BINK_Story *story, bool *can_continue);
int bink_story_cont(BINK_Story *story, char **line, char **err_msg);
int bink_story_get_current_choices(BINK_Story *story, BINK_Choices **choices, size_t *len);
int bink_story_choose_choice_index(BINK_Story *story, size_t choice_index, char **err_msg);
int bink_story_get_current_tags(BINK_Story *story, BINK_Tags **tags, size_t *len);
int bink_story_choose_path_string(BINK_Story *story, char *path, char **err_msg);

void bink_choices_free(BINK_Choices *choices);
int bink_choices_get_text( BINK_Choices *choices, size_t idx, char **text);

void bink_tags_free(BINK_Tags *tags);
int bink_tags_get( BINK_Tags *tags, size_t idx, char **tag);

void bink_cstring_free(char *cstring);

#ifdef __cplusplus
} /* extern "C" */
#endif

#endif /* End of _LIBBINK_H_ */
