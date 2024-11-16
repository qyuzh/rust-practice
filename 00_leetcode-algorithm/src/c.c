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
#include <stdlib.h>

int count_capital(char *word) {
    int cnt = 0;
    for (int i = 0; word[i]; ++i) {
        if (isupper(word[i])) {
            ++cnt;
        }
    }
    return cnt;
}

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

/**
 * @brief p825, leetcode
 *
 * if y meet 0.5 * ages[x] + 7 < ages[y] <= ages[x], then x send a request to y
 *
 * @param ages
 * @param agesSize
 * @return int
 */
int numFriendRequests(int *ages, int agesSize) {
    #define MAX_AGE 121
    int *cnt = (int *)calloc(MAX_AGE, sizeof(int));
    memset(cnt, 0, sizeof(int) * MAX_AGE);
    for (int i = 0; i < agesSize; ++i) {
        ++cnt[ages[i]];
    }

    int *prefix = (int *)calloc(MAX_AGE, sizeof(int));
    memset(prefix, 0, sizeof(int) * MAX_AGE);
    for (int i = 1; i < MAX_AGE; ++i) {
        prefix[i] = prefix[i - 1] + cnt[i];
    }

    int ans = 0;
    for (int i = 15; i <= 120; ++i) {
        if (cnt[i]) {
            int left = i / 2 + 8;
            ans += cnt[i] * (prefix[i] - prefix[left - 1] - 1);
        }
    }

    free(cnt);
    free(prefix);

    return ans;
}
