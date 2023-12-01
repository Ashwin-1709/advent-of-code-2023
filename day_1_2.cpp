#include "bits/stdc++.h"
using namespace std;
#define sz(x) (int)x.size()
#define all(x) begin(x), end(x)

int32_t main() {
    cin.tie(0)->sync_with_stdio(0);
    int ans = 0;
    string s;

    vector<string>dig = {"zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"};
    while(true) {
        cin >> s;
        if(s == "#")
            break;
        vector<int>num;
        for(int i = 0; i < sz(s); i++) {
            if(s[i] >= '0' and s[i] <= '9') 
                num.push_back(s[i] - '0');
            else {
                for(int k = 0; k <= 9; k++) {
                    int nxt = sz(dig[k]) + i - 1;
                    if(nxt < sz(s) and s.substr(i, sz(dig[k])) == dig[k]) 
                        num.push_back(k);
                }
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