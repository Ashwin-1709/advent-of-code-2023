#include "bits/stdc++.h"
using namespace std;
#define sz(x) (int)x.size()
#define all(x) begin(x), end(x)

int32_t main() {
    cin.tie(0)->sync_with_stdio(0);
    int ans = 0;
    string s;
    while(true) {
        cin >> s;
        if(s == "#")
            break;
        vector<int>num;
        for(auto &i: s) {
            if(i >= '0' and i <= '9') {
                num.push_back(i - '0');
            }
        }

        if(sz(num) == 1) {
            ans += 10 * num.front() + num.front();
        } else if(sz(num) >= 2) {
            ans += 10 * num.front() + num.back();
        }
        cout << ans << '\n';
    }
}