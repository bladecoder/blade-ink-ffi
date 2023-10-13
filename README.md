# blade-ink-ffi
A wrapper library providing a C FFI for [Blade Ink](https://crates.io/crates/bladeink). Now you can call Blade Ink Rust library from C!

Here we have an example of use extracted from the test that can be found in `tests/binc_test.c`:

```c
int main(void) {
    uint32_t ret = BINK_OK;
    BINK_Story *story = NULL;
    BINK_Choices *choices = NULL;
    char *err_msg = NULL;
    char *line = NULL;

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
```

For more examples, check the `tests` folder in [this](https://github.com/bladecoder/blade-ink-ffi/tree/main/tests) repository.

Check the [Blade Ink library source code](https://github.com/bladecoder/blade-ink-rs) for more info.

## Download

Compiled packages for the more common platforms containing the library for static and dynamic linking can be downloaded in the [Releases](https://github.com/bladecoder/blade-ink-ffi/releases) section.

If there is no version of the library compiled for the platform you need, you can compile it by yourself:

```bash
$ rustup target add <your_platform_name>
$ cargo build --target <your_platform_name>
```

## Executing tests

We can execute the C tests in the `tests` folder using the `tests/Makefile` file. From the project root folder, execute:

```bash
$ make -f tests/Makefile test
```

By now, tests can only be executed on Linux or Macos and you need to have installed a C tool chain.
