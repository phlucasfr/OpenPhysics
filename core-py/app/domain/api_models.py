from typing import Generic, TypeVar
from pydantic.generics import GenericModel

T = TypeVar('T')


class ApiResponse(GenericModel, Generic[T]):
    data: T | None = None
    message: str
