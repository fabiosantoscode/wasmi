use super::{BrTable, DropKeep, FuncIdx, GlobalIdx, LocalIdx, Offset, SignatureIdx, Target};
use crate::engine::value_stack::StackEntry;

pub trait VisitInstruction {
    type Outcome;

    fn visit_br(&mut self, target: Target) -> Self::Outcome;
    fn visit_br_if_eqz(&mut self, target: Target) -> Self::Outcome;
    fn visit_br_if_nez(&mut self, target: Target) -> Self::Outcome;
    fn visit_return_if_nez(&mut self, drop_keep: DropKeep) -> Self::Outcome;
    fn visit_br_table(&mut self, br_table: BrTable) -> Self::Outcome;
    fn visit_ret(&mut self, drop_keep: DropKeep) -> Self::Outcome;
    fn visit_get_local(&mut self, local_depth: LocalIdx) -> Self::Outcome;
    fn visit_set_local(&mut self, local_depth: LocalIdx) -> Self::Outcome;
    fn visit_tee_local(&mut self, local_depth: LocalIdx) -> Self::Outcome;
    fn visit_get_global(&mut self, global_idx: GlobalIdx) -> Self::Outcome;
    fn visit_set_global(&mut self, global_idx: GlobalIdx) -> Self::Outcome;
    fn visit_call(&mut self, func: FuncIdx) -> Self::Outcome;
    fn visit_call_indirect(&mut self, signature: SignatureIdx) -> Self::Outcome;
    fn visit_const(&mut self, bytes: StackEntry) -> Self::Outcome;
    fn visit_unreachable(&mut self) -> Self::Outcome;
    fn visit_drop(&mut self) -> Self::Outcome;
    fn visit_select(&mut self) -> Self::Outcome;
    fn visit_current_memory(&mut self) -> Self::Outcome;
    fn visit_grow_memory(&mut self) -> Self::Outcome;
    fn visit_i32_load(&mut self, offset: Offset) -> Self::Outcome;
    fn visit_i64_load(&mut self, offset: Offset) -> Self::Outcome;
    fn visit_f32_load(&mut self, offset: Offset) -> Self::Outcome;
    fn visit_f64_load(&mut self, offset: Offset) -> Self::Outcome;
    fn visit_i32_load_i8(&mut self, offset: Offset) -> Self::Outcome;
    fn visit_i32_load_u8(&mut self, offset: Offset) -> Self::Outcome;
    fn visit_i32_load_i16(&mut self, offset: Offset) -> Self::Outcome;
    fn visit_i32_load_u16(&mut self, offset: Offset) -> Self::Outcome;
    fn visit_i64_load_i8(&mut self, offset: Offset) -> Self::Outcome;
    fn visit_i64_load_u8(&mut self, offset: Offset) -> Self::Outcome;
    fn visit_i64_load_i16(&mut self, offset: Offset) -> Self::Outcome;
    fn visit_i64_load_u16(&mut self, offset: Offset) -> Self::Outcome;
    fn visit_i64_load_i32(&mut self, offset: Offset) -> Self::Outcome;
    fn visit_i64_load_u32(&mut self, offset: Offset) -> Self::Outcome;
    fn visit_i32_store(&mut self, offset: Offset) -> Self::Outcome;
    fn visit_i64_store(&mut self, offset: Offset) -> Self::Outcome;
    fn visit_f32_store(&mut self, offset: Offset) -> Self::Outcome;
    fn visit_f64_store(&mut self, offset: Offset) -> Self::Outcome;
    fn visit_i32_store_8(&mut self, offset: Offset) -> Self::Outcome;
    fn visit_i32_store_16(&mut self, offset: Offset) -> Self::Outcome;
    fn visit_i64_store_8(&mut self, offset: Offset) -> Self::Outcome;
    fn visit_i64_store_16(&mut self, offset: Offset) -> Self::Outcome;
    fn visit_i64_store_32(&mut self, offset: Offset) -> Self::Outcome;
    fn visit_i32_eqz(&mut self) -> Self::Outcome;
    fn visit_i32_eq(&mut self) -> Self::Outcome;
    fn visit_i32_ne(&mut self) -> Self::Outcome;
    fn visit_i32_lt_s(&mut self) -> Self::Outcome;
    fn visit_i32_lt_u(&mut self) -> Self::Outcome;
    fn visit_i32_gt_s(&mut self) -> Self::Outcome;
    fn visit_i32_gt_u(&mut self) -> Self::Outcome;
    fn visit_i32_le_s(&mut self) -> Self::Outcome;
    fn visit_i32_le_u(&mut self) -> Self::Outcome;
    fn visit_i32_ge_s(&mut self) -> Self::Outcome;
    fn visit_i32_ge_u(&mut self) -> Self::Outcome;
    fn visit_i64_eqz(&mut self) -> Self::Outcome;
    fn visit_i64_eq(&mut self) -> Self::Outcome;
    fn visit_i64_ne(&mut self) -> Self::Outcome;
    fn visit_i64_lt_s(&mut self) -> Self::Outcome;
    fn visit_i64_lt_u(&mut self) -> Self::Outcome;
    fn visit_i64_gt_s(&mut self) -> Self::Outcome;
    fn visit_i64_gt_u(&mut self) -> Self::Outcome;
    fn visit_i64_le_s(&mut self) -> Self::Outcome;
    fn visit_i64_le_u(&mut self) -> Self::Outcome;
    fn visit_i64_ge_s(&mut self) -> Self::Outcome;
    fn visit_i64_ge_u(&mut self) -> Self::Outcome;
    fn visit_f32_eq(&mut self) -> Self::Outcome;
    fn visit_f32_ne(&mut self) -> Self::Outcome;
    fn visit_f32_lt(&mut self) -> Self::Outcome;
    fn visit_f32_gt(&mut self) -> Self::Outcome;
    fn visit_f32_le(&mut self) -> Self::Outcome;
    fn visit_f32_ge(&mut self) -> Self::Outcome;
    fn visit_f64_eq(&mut self) -> Self::Outcome;
    fn visit_f64_ne(&mut self) -> Self::Outcome;
    fn visit_f64_lt(&mut self) -> Self::Outcome;
    fn visit_f64_gt(&mut self) -> Self::Outcome;
    fn visit_f64_le(&mut self) -> Self::Outcome;
    fn visit_f64_ge(&mut self) -> Self::Outcome;
    fn visit_i32_clz(&mut self) -> Self::Outcome;
    fn visit_i32_ctz(&mut self) -> Self::Outcome;
    fn visit_i32_popcnt(&mut self) -> Self::Outcome;
    fn visit_i32_add(&mut self) -> Self::Outcome;
    fn visit_i32_sub(&mut self) -> Self::Outcome;
    fn visit_i32_mul(&mut self) -> Self::Outcome;
    fn visit_i32_div_s(&mut self) -> Self::Outcome;
    fn visit_i32_div_u(&mut self) -> Self::Outcome;
    fn visit_i32_rem_s(&mut self) -> Self::Outcome;
    fn visit_i32_rem_u(&mut self) -> Self::Outcome;
    fn visit_i32_and(&mut self) -> Self::Outcome;
    fn visit_i32_or(&mut self) -> Self::Outcome;
    fn visit_i32_xor(&mut self) -> Self::Outcome;
    fn visit_i32_shl(&mut self) -> Self::Outcome;
    fn visit_i32_shr_s(&mut self) -> Self::Outcome;
    fn visit_i32_shr_u(&mut self) -> Self::Outcome;
    fn visit_i32_rotl(&mut self) -> Self::Outcome;
    fn visit_i32_rotr(&mut self) -> Self::Outcome;
    fn visit_i64_clz(&mut self) -> Self::Outcome;
    fn visit_i64_ctz(&mut self) -> Self::Outcome;
    fn visit_i64_popcnt(&mut self) -> Self::Outcome;
    fn visit_i64_add(&mut self) -> Self::Outcome;
    fn visit_i64_sub(&mut self) -> Self::Outcome;
    fn visit_i64_mul(&mut self) -> Self::Outcome;
    fn visit_i64_div_s(&mut self) -> Self::Outcome;
    fn visit_i64_div_u(&mut self) -> Self::Outcome;
    fn visit_i64_rem_s(&mut self) -> Self::Outcome;
    fn visit_i64_rem_u(&mut self) -> Self::Outcome;
    fn visit_i64_and(&mut self) -> Self::Outcome;
    fn visit_i64_or(&mut self) -> Self::Outcome;
    fn visit_i64_xor(&mut self) -> Self::Outcome;
    fn visit_i64_shl(&mut self) -> Self::Outcome;
    fn visit_i64_shr_s(&mut self) -> Self::Outcome;
    fn visit_i64_shr_u(&mut self) -> Self::Outcome;
    fn visit_i64_rotl(&mut self) -> Self::Outcome;
    fn visit_i64_rotr(&mut self) -> Self::Outcome;
    fn visit_f32_abs(&mut self) -> Self::Outcome;
    fn visit_f32_neg(&mut self) -> Self::Outcome;
    fn visit_f32_ceil(&mut self) -> Self::Outcome;
    fn visit_f32_floor(&mut self) -> Self::Outcome;
    fn visit_f32_trunc(&mut self) -> Self::Outcome;
    fn visit_f32_nearest(&mut self) -> Self::Outcome;
    fn visit_f32_sqrt(&mut self) -> Self::Outcome;
    fn visit_f32_add(&mut self) -> Self::Outcome;
    fn visit_f32_sub(&mut self) -> Self::Outcome;
    fn visit_f32_mul(&mut self) -> Self::Outcome;
    fn visit_f32_div(&mut self) -> Self::Outcome;
    fn visit_f32_min(&mut self) -> Self::Outcome;
    fn visit_f32_max(&mut self) -> Self::Outcome;
    fn visit_f32_copysign(&mut self) -> Self::Outcome;
    fn visit_f64_abs(&mut self) -> Self::Outcome;
    fn visit_f64_neg(&mut self) -> Self::Outcome;
    fn visit_f64_ceil(&mut self) -> Self::Outcome;
    fn visit_f64_floor(&mut self) -> Self::Outcome;
    fn visit_f64_trunc(&mut self) -> Self::Outcome;
    fn visit_f64_nearest(&mut self) -> Self::Outcome;
    fn visit_f64_sqrt(&mut self) -> Self::Outcome;
    fn visit_f64_add(&mut self) -> Self::Outcome;
    fn visit_f64_sub(&mut self) -> Self::Outcome;
    fn visit_f64_mul(&mut self) -> Self::Outcome;
    fn visit_f64_div(&mut self) -> Self::Outcome;
    fn visit_f64_min(&mut self) -> Self::Outcome;
    fn visit_f64_max(&mut self) -> Self::Outcome;
    fn visit_f64_copysign(&mut self) -> Self::Outcome;
    fn visit_i32_wrap_i64(&mut self) -> Self::Outcome;
    fn visit_i32_trunc_f32(&mut self) -> Self::Outcome;
    fn visit_u32_trunc_f32(&mut self) -> Self::Outcome;
    fn visit_i32_trunc_f64(&mut self) -> Self::Outcome;
    fn visit_u32_trunc_f64(&mut self) -> Self::Outcome;
    fn visit_i64_extend_i32(&mut self) -> Self::Outcome;
    fn visit_i64_extend_u32(&mut self) -> Self::Outcome;
    fn visit_i64_trunc_f32(&mut self) -> Self::Outcome;
    fn visit_u64_trunc_f32(&mut self) -> Self::Outcome;
    fn visit_i64_trunc_f64(&mut self) -> Self::Outcome;
    fn visit_u64_trunc_f64(&mut self) -> Self::Outcome;
    fn visit_f32_convert_i32(&mut self) -> Self::Outcome;
    fn visit_f32_convert_u32(&mut self) -> Self::Outcome;
    fn visit_f32_convert_i64(&mut self) -> Self::Outcome;
    fn visit_f32_convert_u64(&mut self) -> Self::Outcome;
    fn visit_f32_demote_f64(&mut self) -> Self::Outcome;
    fn visit_f64_convert_i32(&mut self) -> Self::Outcome;
    fn visit_f64_convert_u32(&mut self) -> Self::Outcome;
    fn visit_f64_convert_i64(&mut self) -> Self::Outcome;
    fn visit_f64_convert_u64(&mut self) -> Self::Outcome;
    fn visit_f64_promote_f32(&mut self) -> Self::Outcome;
    fn visit_i32_reinterpret_f32(&mut self) -> Self::Outcome;
    fn visit_i64_reinterpret_f64(&mut self) -> Self::Outcome;
    fn visit_f32_reinterpret_i32(&mut self) -> Self::Outcome;
    fn visit_f64_reinterpret_i64(&mut self) -> Self::Outcome;

    fn visit_i32_trunc_sat_f32(&mut self) -> Self::Outcome;
    fn visit_u32_trunc_sat_f32(&mut self) -> Self::Outcome;
    fn visit_i32_trunc_sat_f64(&mut self) -> Self::Outcome;
    fn visit_u32_trunc_sat_f64(&mut self) -> Self::Outcome;
    fn visit_i64_trunc_sat_f32(&mut self) -> Self::Outcome;
    fn visit_u64_trunc_sat_f32(&mut self) -> Self::Outcome;
    fn visit_i64_trunc_sat_f64(&mut self) -> Self::Outcome;
    fn visit_u64_trunc_sat_f64(&mut self) -> Self::Outcome;
}
