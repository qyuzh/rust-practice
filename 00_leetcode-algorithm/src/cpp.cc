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

// P1883
int minSkips(vector<int> &dist, int speed, int hoursBefore) {
    if (accumulate(dist.begin(), dist.end(), 0) >
        (long long)speed * hoursBefore) {
        return -1;
    }

    int n = dist.size();

    vector<vector<int>> memo(n, vector<int>(n, -1));

    // dfs(i,j) := (在最多跳过i次的情况下, 从dist[0]到dist[j]需要的最小时间) *
    // speed
    function<int(int, int)> dfs = [&](int i, int j) -> int {
        if (j < 0) {
            return 0;
        }

        auto &res = memo[i][j];

        if (res != -1) {
            return res;
        }

        // 本次不跳过
        res = (dfs(i, j - 1) + dist[j] + speed - 1) / speed * speed;

        // 本次跳过
        if (i > 0) {
            res = min(res, dfs(i - 1, j - 1) + dist[j]);
        }

        return res;
    };

    for (int i = 0;; ++i) {
        if (dfs(i, n - 2) + dist[n - 1] <= (long long)speed * hoursBefore) {
            return i;
        }
    }
}

int main() { return 0; }