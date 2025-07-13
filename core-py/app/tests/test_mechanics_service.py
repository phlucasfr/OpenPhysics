from app.domain.mechanics_models import VectorVelocityRequest
from app.services.mechanics_service import vector_velocity


def test_vector_velocity_basic():
    data = VectorVelocityRequest(
        direction="north-east",
        displacement_change=100.0,
        displacement_unit="meters",
        time_change=10.0,
        time_unit="seconds"
    )

    result = vector_velocity(data)

    assert result.velocity == 10.0
    assert result.direction == "north-east"
    assert result.velocity_unit == "meters/seconds"


def test_vector_velocity_zero_time():
    data = VectorVelocityRequest(
        direction="east",
        displacement_change=50.0,
        displacement_unit="meters",
        time_change=0.0,
        time_unit="seconds"
    )

    try:
        vector_velocity(data)
        assert False, "Expected ZeroDivisionError"
    except ZeroDivisionError:
        assert True
