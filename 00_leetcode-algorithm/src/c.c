/**
 * @file c.c
 * @author qyuzh (qyuzhang@139.com)
 * @brief
 * @version 0.1
 * @date 2024-06-23
 *
 * @copyright Copyright (c) 2024
 *
 */
#include <ctype.h>
#include <stdbool.h>
#include <string.h>

/**
 * @brief p520, leetcode
 *
 * @param word
 * @return true
 * @return false
 */
bool detectCapitalUse(char *word) {
    int cnt = count_capital(word);
    return cnt == 0 || cnt == strlen(word) || cnt == 1 && isupper(word[0]);
}

int count_capital(char *word) {
    int cnt = 0;
    for (int i = 0; word[i]; ++i) {
        if (isupper(word[i])) {
            ++cnt;
        }
    }
    return cnt;
}