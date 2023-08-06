#include <bits/stdc++.h>
#define ff first
#define ss second
#define ll long long
#define ld long double
#define pb push_back
#define eb emplace_back
#define mp make_pair
#define mt make_tuple
#define pii pair<int, int>
#define vi vector<int>
#define vl vector<ll>
#define vii vector<pii>
#define sws ios_base::sync_with_stdio(false);cin.tie(NULL);cout.tie(NULL);
#define endl '\n'
#define teto(a, b) ((a+b-1)/(b))
using namespace std;

const int MAX = 300020;
const ll MOD = 998244353;
const int INF = 1e9;
const ll LLINF = 0x3f3f3f3f3f3f3f3f;
const ld EPS = 1e-7;

// Extra
#define forn(i, n) for(int i = 0; i < (int)n; i++)
#define forne(i, a, b) for(int i = a; i <= b; i++)
#define all(x) x.begin(), x.end()
#define dbg(msg, var) cout << msg << " " << var << endl;
//

struct FT {
    vector<int> bit;  // indexado em 0
    int n;

    FT(int n) {
        this->n = n+1;
        bit.assign(n+1, 0);
    }

    int sum(int idx) {
        int ret = 0;
        for (; idx > 0; idx -= idx & -idx)
            ret += bit[idx];
        return ret;
    }

    int sum(int l, int r) { // [l, r]
        return sum(r) - sum(l - 1);
    }

    void add(int idx, int delta) {
        for (; idx < n; idx += idx & -idx)
            bit[idx] += delta;
    }
};

vi z_algo(const string &s)
{   // returns vector for each idx where a prefix of size i starts.
    int n = s.size();
    int L = 0, R = 0;
    vi z(n, 0);
    for(int i = 1; i < n; i++){
        if(i <= R)
            z[i] = min(z[i-L], R - i + 1);
        while(z[i]+i < n and s[ z[i]+i ] == s[ z[i] ])
            z[i]++;
        if(i+z[i]-1 > R){
            L = i;
            R = i + z[i] - 1;
        }
    }
    return z;
}

int main() {
    sws;
    string s;
    cin >> s;

    vector<pii> res;
    string t = s + "$" + s;
    vi Z = z_algo(t);

    FT ft = FT(s.size()+1);

    for(int i = s.size(); i < (int)Z.size(); i++) {
        if(Z[i] > 0) {
            ft.add(1, 1);
            ft.add(Z[i]+1, -1);
        }
    }

    for(int i = Z.size()-1; i > (int)s.size(); i--) {
        int sz = Z.size()-i;
        if(Z[i] >= sz) {
            res.pb(mp(sz, ft.sum(0, sz)));
        }
    }

    sort(all(res));
    cout << res.size() << endl;
    for(auto p : res)
        cout << p.ff << " " << p.ss << endl;

    return 0;
}
