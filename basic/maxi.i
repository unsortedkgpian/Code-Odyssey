# 0 "maxifuntion.cpp"
# 0 "<built-in>"
# 0 "<command-line>"
# 1 "/usr/include/stdc-predef.h" 1 3 4
# 0 "<command-line>" 2
# 1 "maxifuntion.cpp"


int arr[100];

int max(int a, int b){
    if(a>b) return a;
    else return b;
}

int main(){
    int x = 5;
    int y=6;
    arr[0] = max(x,y);
}
