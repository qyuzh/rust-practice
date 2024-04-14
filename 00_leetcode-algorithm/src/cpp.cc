#include <bits/stdc++.h>

using namespace std;

// Weekly Contest 390, C
vector<long long> mostFrequentIDs(vector<int> &nums, vector<int> &freq) {
    // cnt[i] := the freq of i
    unordered_map<int, long long> cnt;

    multiset<long long> m; // 有序集合

    int n = nums.size();

    vector<long long> ans(n);
    for (int i = 0; i < n; i++) {
        int x = nums[i];
        auto it = m.find(cnt[x]); // log(n)
        if (it != m.end()) {
            m.erase(it);
        }
        cnt[x] += freq[i];
        m.insert(cnt[x]);
        ans[i] = *m.rbegin();
    }

    return ans;
}

// weekly contest 393, C
long long findKthSmallest(vector<int> &coins, int k) {
    auto check = [&](long long m) -> bool {
        long long cnt = 0;
        for (int mask = 1; mask < (1 << coins.size()); ++mask) {
            long long lcm_res = 1;
            for (int j = 0; j < coins.size(); ++j) {
                if (mask >> j & 1) {
                    lcm_res = lcm(lcm_res, coins[j]);
                    if (lcm_res > m) {
                        break;
                    }
                }
            }

            // GCC compiler-specific built-in function to count the number of
            // set bits in the binary representation of an unsigned integer
            cnt += __builtin_popcount(mask) % 2 ? m / lcm_res : -m / lcm_res;
        }

        return cnt >= k; // means that
    };

    // binary search
    long long l = k;
    long long r = (long long)ranges::min(coins) * k;
    while (l < r) {
        long long mid = (l + r) >> 1;
        if (check(mid)) {
            r = mid;
        } else {
            l = mid + 1;
        }
    }

    return l;
}

int main() { return 0; }