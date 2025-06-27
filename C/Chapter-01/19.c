#include<stdio.h>
#define MAXLINE 1000

int get_line(char line[], int maxline);
void copy(char to[], char from[]);

int main(){
    int len;
    char line[MAXLINE];
    char ans[MAXLINE][MAXLINE];
    int j=0;

    while((len = get_line(line, MAXLINE))>0)
        if(len>=80){
            copy(ans[j],line );
            j++;
        }

    for(int i=0;i<j;i++){
        printf("%s", ans[i]);
    }

}

int get_line(char s[], int lim){
    int c,i;
    for(i=0;i<lim-1 && (c=getchar()) !=EOF && c!='\n'; ++i)
        s[i] =c;
    if(c=='\n'){
        s[i]=c;
        ++i;
    }
    s[i] ='\0';
    return i;
}

void copy(char to[], char from[]){
    int i;
    i=0;
    while((to[i] = from[i]) != '\0')
        ++i;
}
