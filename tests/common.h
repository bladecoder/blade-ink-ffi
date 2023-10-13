#ifndef _COMMON_H_
#define _COMMON_H_

#ifdef __cplusplus
extern "C" {
#endif

void finish(int rc, BINK_Story *story, char *err_msg);
void check_ret(int ret, BINK_Story *story, char *err_msg);
void print_choices(BINK_Choices *choices, size_t len);
char* read_json_file(const char* filename);
void assert_cont(BINK_Story *story, char *expected);

#ifdef __cplusplus
} /* extern "C" */
#endif

#endif /* End of _COMMON_H_ */
