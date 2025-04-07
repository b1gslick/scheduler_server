from pydantic import BaseModel


class ActivityType(BaseModel):
    title: str
    content: str
    time: int


class Account(BaseModel):
    email: str
    password: str


class LoginResponse(BaseModel):
    token: str
