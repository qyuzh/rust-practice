#include <bits/stdc++.h>

using namespace std;

class Solution2663 {
  public:
    string smallestBeautifulString(string s, int k) {
        k += 'a';
        const int n = s.length();
        int i = n - 1;
        s[i] += 1;
        while (i < n) {
            if (s[i] == k) {
                if (i == 0) {
                    return "";
                }
                s[i] = 'a';
                s[--i]++;
            } else if (i && s[i] == s[i - 1] || s[i] == s[i - 2]) {
                s[i] += 1;
            } else {
                i += 1;
            }
        }
        return s;
    }
};

class Solution312 {
    vector<vector<int>> memo;
    vector<int> val;

  public:
    int maxCoins(vector<int> &nums) {
        int n = nums.size();

        val.resize(n + 2);
        for (int i = 0; i <= n; ++i) {
            val[i] = nums[i - 1];
        }
        val[0] = val[n + 1] = 1;

        memo.resize(n + 2, vector<int>(n + 2, -1));

        return solve(0, n + 1);
    }

    int solve(int l, int r) {
        if (left >= right - 1) {
            return 0;
        }

        if (memo[l][r] != -1) {
            return memo[l][r];
        }

        for (int i = l + 1; i < r; ++i) {
            // the balloon in i-th position is the first balloon that was burst
            // in [l, r]
            int sum = val[l] * val[i] * val[r];

            sum += solve(l, i) + solve(i, r);
            memo[l][r] = max(memo[l][r], sum);
        }

        return memo[l][r];
    }
};

vector<int> findPermutation(vector<int> &nums) {
    int n = nums.size();

    vector<int> ids;
    ids.reserve(n);
    for (int i = 0; i < n; ++i) {
        ids[i] = i;
    }

    int t = INT32_MAX;

    vector<int> ret;

    do {
        int score = 0;

        for (int i = 0; i < n; ++i) {
            score += abs(ids[i] - nums[ids[(i + 1) % n]]);
        }

        if (score < t) {
            t = score;
            ret = ids;
        }

        if (score == 0) {
            break;
        }

    } while (std::next_permutation(ids.begin(), ids.end()));

    return ret;
}

// Weekly Contest 396, D
int minCostToEqualizeArray(vector<int> &nums, int cost1, int cost2) {
    int n = nums.size();

    auto mxv = *max_element(nums.begin(), nums.end()); // max value

    long long total_det = 0;
    int mxv_det = 0;
    for (int x : nums) {
        int det = mxv - x;
        total_det += det;
        mxv_det = max(mxv_det, det);
    }

    const int MOD = 1e9 + 7;

    if (nums.size() <= 2 || cost1 * 2 <= cost2) {
        return total_det * cost1 % MOD;
    }

    long long ans = 1e18;
    for (int i = mxv, tf = 0; tf < 2; ++i) {
        long long t = 0;

        if (mxv_det > total_det - mxv_det) {
            t = (total_det - mxv_det) * cost2;
            long long r = mxv_det - (total_det - mxv_det);
            t += r * cost1;
        } else {
            t = total_det / 2 * cost2;
            if (total_det & 1) {
                t += cost1;
            }
            tf += 1;
        }

        ans = min(ans, t);
        total_det += n;
        mxv_det += 1;
    }

    return ans % MOD;
}

// refer to
// https://leetcode.cn/problems/minimum-cost-to-equalize-array/solutions/2765988/yi-bu-yi-bu-zhao-chu-ti-mu-de-zui-you-ji-cprv/
int minCostToEqualizeArray2(vector<int> &nums, int cost1, int cost2) {
    long long n = nums.size();
    const int MOD = 1e9 + 7;

    long long M = 0, m = 0, s = 0;
    for (int x : nums) {
        M = max(M, (long long)x);
    }

    for (int x : nums) {
        m = max(m, (long long)(M - x));
        s += M - x;
    }

    if (n <= 2ll || cost2 >= cost1 * 2) {
        return s * cost1 % MOD;
    }

    auto calc = [&](long long m, long long s) {
        if (m > s - m) {
            return (s - m) * cost2 + (2ll * m - s) * cost1;
        } else {
            return (s / 2) * cost2 + (s % 2) * cost1;
        }
    };

    long long ret = min(calc(m, s), calc(m + 1ll, s + n));
    long long v = (2 * m - s) / (n - 2);
    for (long long d = max(0ll, v - 2); d <= v + 2; ++d) {
        ret = min(ret, calc(m + d, s + n * d));
    }

    return ret % MOD;
}

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