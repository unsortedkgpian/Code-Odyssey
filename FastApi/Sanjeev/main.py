from typing import Union
from fastapi import FastAPI
from fastapi.params import Body
from pydantic import BaseModel


app = FastAPI()

class PayloadModel(BaseModel):
    title:str
    content:str
    id:int

# @app.get("/") -> decorators
@app.get("/")
def dead_root():
    return {"Hello":"Word"}


@app.get("/items/{item_id}")
def read_item(item_id: int, q: Union[str, None] = None):
    return {"item_id": item_id, "q": q}


@app.get("/post")
def get_post():
    return{"data":"post"}

@app.post("/post")
def post_the_post(payLoad:PayloadModel):
    print(payLoad)
    return {"succes":"true"}

my_post=[{"id":1,"title":"how are you", "content":"this is good",}, {"id":2,"title":"I am good", "content":"this is not good"}]

# CRUD -> data
@app.get("/posts")
def get_all_posts():
    return {"All_post":my_post}


@app.get("/post/{post_id}")
def get_post(post_id:int):
    for post in my_post:
        if post["id"] == post_id:
            return post
        
    return {"error":"Post not found"}

@app.post("/posts")
def make_post(post:PayloadModel)
