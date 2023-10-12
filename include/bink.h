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

struct bink_story;
struct bink_choices;
struct bink_tags;

int bink_story_new(struct bink_story **story, char *json_string, char **err_msg);
void bink_story_free(struct bink_story *story);
int bink_story_can_continue(struct bink_story *story, bool *can_continue);
int bink_story_cont(struct bink_story *story, char **line, char **err_msg);
int bink_story_get_current_choices(struct bink_story *story, struct bink_choices **choices, size_t *len);
int bink_story_choose_choice_index(struct bink_story *story, size_t choice_index);
int bink_story_get_current_tags(struct bink_story *story, struct bink_tags **tags, size_t *len);

void bink_choices_free(struct bink_choices *choices);
int bink_choices_get_text( struct bink_choices *choices, size_t idx, char **text);

void bink_tags_free(struct bink_tags *tags);
int bink_tags_get( struct bink_tags *tags, size_t idx, char **tag);

void bink_cstring_free(char *cstring);

#ifdef __cplusplus
} /* extern "C" */
#endif

#endif /* End of _LIBBINK_H_ */
