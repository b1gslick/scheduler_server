from pydantic import BaseModel


class ActivityType(BaseModel):
    content: str
    time: int
    title: str
    id: int | None = None


class Account(BaseModel):
    email: str
    password: str


class LoginResponse(BaseModel):
    token: str
