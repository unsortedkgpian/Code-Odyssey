#define MAXN 100

int arr[MAXN];

int max(int a, int b){
    if(a>b) return a;
    else return b;
}

int main(){
    int x = 5;
    int y=6;
    arr[0] = max(x,y);
}
