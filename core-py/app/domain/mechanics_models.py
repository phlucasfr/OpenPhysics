from pydantic import BaseModel


class VectorVelocityRequest(BaseModel):
    direction: str
    displacement_change: float
    displacement_unit: str
    time_change: float
    time_unit: str


class VectorVelocityResponse(BaseModel):
    velocity: float
    direction: str
    velocity_unit: str
