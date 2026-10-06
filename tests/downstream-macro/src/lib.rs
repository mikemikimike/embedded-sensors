#![no_std]

use embedded_sensors_hal::decl_threshold_traits;
use embedded_sensors_hal::sensor::ErrorType;

pub struct BlockingSensor;

impl ErrorType for BlockingSensor {
    type Error = core::convert::Infallible;
}

pub trait BlockingSensorTrait: ErrorType<Error = core::convert::Infallible> {}

impl BlockingSensorTrait for BlockingSensor {}

impl<T: BlockingSensorTrait + ?Sized> BlockingSensorTrait for &mut T {}

decl_threshold_traits!(blocking, BlockingSensor, BlockingSensorTrait, f32, "units");

pub struct AsyncSensor;

impl ErrorType for AsyncSensor {
    type Error = core::convert::Infallible;
}

pub trait AsyncSensorTrait: ErrorType<Error = core::convert::Infallible> {}

impl AsyncSensorTrait for AsyncSensor {}

impl<T: AsyncSensorTrait + ?Sized> AsyncSensorTrait for &mut T {}

decl_threshold_traits!(async, AsyncSensor, AsyncSensorTrait, f32, "units");

pub fn require_blocking_traits<T: BlockingSensorThresholdSet + BlockingSensorHysteresis>() {}

pub fn require_async_traits<
    T: AsyncSensorThresholdSet + AsyncSensorHysteresis + AsyncSensorThresholdWait,
>() {
}
