use anchor_lang::prelude::*;

use crate::state::Store;

#[derive(Accounts)]
pub struct AddToStore<'info> {
    #[account(
        mut,
        seeds=[b"0".as_ref()],
        bump,
    )]
    pub store_0: Box<Account<'info, Store>>,
    #[account(
        mut,
        seeds=[b"1".as_ref()],
        bump,
    )]
    pub store_1: Box<Account<'info, Store>>,
    #[account(
        mut,
        seeds=[b"2".as_ref()],
        bump,
    )]
    pub store_2: Box<Account<'info, Store>>,
    #[account(
        mut,
        seeds=[b"3".as_ref()],
        bump,
        )]
    pub store_3: Box<Account<'info, Store>>,
    #[account(
        mut,
        seeds=[b"4".as_ref()],
        bump,
    )]
    pub store_4: Box<Account<'info, Store>>,
    #[account(
        mut,
        seeds=[b"5".as_ref()],
        bump,
    )]
    pub store_5: Box<Account<'info, Store>>,
    #[account(
        mut,
        seeds=[b"6".as_ref()],
        bump,
    )]
    pub store_6: Box<Account<'info, Store>>,
    #[account(
        mut,
        seeds=[b"7".as_ref()],
        bump,
    )]
    pub store_7: Box<Account<'info, Store>>,
    #[account(
        mut,
        seeds=[b"8".as_ref()],
        bump,
    )]
    pub store_8: Box<Account<'info, Store>>,
    #[account(
        mut,
        seeds=[b"9".as_ref()],
        bump,
    )]
    pub store_9: Box<Account<'info, Store>>,
    #[account(
        mut,
        seeds=[b"10".as_ref()],
        bump,
    )]
    pub store_10: Box<Account<'info, Store>>,
    #[account(
        mut,
        seeds=[b"11".as_ref()],
        bump,
    )]
    pub store_11: Box<Account<'info, Store>>,
    #[account(
        mut,
        seeds=[b"12".as_ref()],
        bump,
    )]
    pub store_12: Box<Account<'info, Store>>,
    #[account(
        mut,
        seeds=[b"13".as_ref()],
        bump,
    )]
    pub store_13: Box<Account<'info, Store>>,
    #[account(
        mut,
        seeds=[b"14".as_ref()],
        bump,
    )]
    pub store_14: Box<Account<'info, Store>>,
    #[account(
        mut,
        seeds=[b"15".as_ref()],
        bump,
    )]
    pub store_15: Box<Account<'info, Store>>,
    #[account(
        mut,
        seeds=[b"16".as_ref()],
        bump,
    )]
    pub store_16: Box<Account<'info, Store>>,
    #[account(
        mut,
        seeds=[b"17".as_ref()],
        bump,
    )]
    pub store_17: Box<Account<'info, Store>>,
    #[account(
      mut,
        seeds=[b"18".as_ref()],
        bump,
    )]
    pub store_18: Box<Account<'info, Store>>,
    #[account(mut,
        seeds=[b"19".as_ref()],
        bump,
    )]
    pub store_19: Box<Account<'info, Store>>,
    #[account(mut,
        seeds=[b"20".as_ref()],
        bump,
    )]
    pub store_20: Box<Account<'info, Store>>,
    #[account(mut,
        seeds=[b"21".as_ref()],
        bump,
    )]
    pub store_21: Box<Account<'info, Store>>,
    #[account(mut,
        seeds=[b"22".as_ref()],
        bump,
    )]
    pub store_22: Box<Account<'info, Store>>,
    #[account(mut,
        seeds=[b"23".as_ref()],
        bump,
    )]
    pub store_23: Box<Account<'info, Store>>,
    #[account(mut,
        seeds=[b"24".as_ref()],
        bump,
    )]
    pub store_24: Box<Account<'info, Store>>,
    #[account(mut,
        seeds=[b"25".as_ref()],
        bump,
    )]
    pub store_25: Box<Account<'info, Store>>,
    #[account(mut,
        seeds=[b"26".as_ref()],
        bump,
    )]
    pub store_26: Box<Account<'info, Store>>,
    #[account(mut,
        seeds=[b"27".as_ref()],
        bump,
    )]
    pub store_27: Box<Account<'info, Store>>,
    #[account(mut,
        seeds=[b"28".as_ref()],
        bump,
    )]
    pub store_28: Box<Account<'info, Store>>,
    #[account(mut,
        seeds=[b"29".as_ref()],
        bump,
    )]
    pub store_29: Box<Account<'info, Store>>,
    #[account(mut,
        seeds=[b"30".as_ref()],
        bump,
    )]
    pub store_30: Box<Account<'info, Store>>,
    #[account(mut,
        seeds=[b"31".as_ref()],
        bump,
    )]
    pub store_31: Box<Account<'info, Store>>,
    #[account(mut,
        seeds=[b"32".as_ref()],
        bump,
    )]
    pub store_32: Box<Account<'info, Store>>,
    #[account(mut,
        seeds=[b"33".as_ref()],
        bump,
    )]
    pub store_33: Box<Account<'info, Store>>,
    #[account(mut,
        seeds=[b"34".as_ref()],
        bump,
    )]
    pub store_34: Box<Account<'info, Store>>,
    #[account(mut,
        seeds=[b"35".as_ref()],
        bump,
    )]
    pub store_35: Box<Account<'info, Store>>,
    #[account(mut,
        seeds=[b"36".as_ref()],
        bump,
    )]
    pub store_36: Box<Account<'info, Store>>,
    #[account(mut,
        seeds=[b"37".as_ref()],
        bump,
    )]
    pub store_37: Box<Account<'info, Store>>,
    #[account(mut,
        seeds=[b"38".as_ref()],
        bump,
    )]
    pub store_38: Box<Account<'info, Store>>,
    #[account(mut,
        seeds=[b"39".as_ref()],
        bump,
    )]
    pub store_39: Box<Account<'info, Store>>,
    #[account(mut,
        seeds=[b"40".as_ref()],
        bump,
    )]
    pub store_40: Box<Account<'info, Store>>,
    #[account(mut,
        seeds=[b"41".as_ref()],
        bump,
    )]
    pub store_41: Box<Account<'info, Store>>,
    #[account(mut,
        seeds=[b"42".as_ref()],
        bump,
    )]
    pub store_42: Box<Account<'info, Store>>,
    #[account(mut,
        seeds=[b"43".as_ref()],
        bump,
    )]
    pub store_43: Box<Account<'info, Store>>,
    #[account(mut,
        seeds=[b"44".as_ref()],
        bump,
    )]
    pub store_44: Box<Account<'info, Store>>,
    #[account(mut,
        seeds=[b"45".as_ref()],
        bump,
    )]
    pub store_45: Box<Account<'info, Store>>,
    #[account(mut,
        seeds=[b"46".as_ref()],
        bump,
    )]
    pub store_46: Box<Account<'info, Store>>,
    #[account(mut,
        seeds=[b"47".as_ref()],
        bump,
    )]
    pub store_47: Box<Account<'info, Store>>,
    #[account(mut,
        seeds=[b"48".as_ref()],
        bump,
    )]
    pub store_48: Box<Account<'info, Store>>,
    #[account(mut,
        seeds=[b"49".as_ref()],
        bump,
    )]
    pub store_49: Box<Account<'info, Store>>,
    #[account(mut,
        seeds=[b"50".as_ref()],
        bump,
    )]
    pub store_50: Box<Account<'info, Store>>,
    #[account(mut,
        seeds=[b"51".as_ref()],
        bump,
    )]
    pub store_51: Box<Account<'info, Store>>,
    #[account(mut,
        seeds=[b"52".as_ref()],
        bump,
    )]
    pub store_52: Box<Account<'info, Store>>,
    #[account(mut,
        seeds=[b"53".as_ref()],
        bump,
    )]
    pub store_53: Box<Account<'info, Store>>,
    #[account(mut,
        seeds=[b"54".as_ref()],
        bump,
    )]
    pub store_54: Box<Account<'info, Store>>,
    #[account(mut,
        seeds=[b"55".as_ref()],
        bump,
    )]
    pub store_55: Box<Account<'info, Store>>,
    #[account(mut,
        seeds=[b"56".as_ref()],
        bump,
    )]
    pub store_56: Box<Account<'info, Store>>,
    #[account(mut,
        seeds=[b"57".as_ref()],
        bump,
    )]
    pub store_57: Box<Account<'info, Store>>,
    #[account(mut,
        seeds=[b"58".as_ref()],
        bump,
    )]
    pub store_58: Box<Account<'info, Store>>,
    #[account(mut,
        seeds=[b"59".as_ref()],
        bump,
    )]
    pub store_59: Box<Account<'info, Store>>, 
}

pub fn handler(ctx: Context<AddToStore>, data: u8) -> Result<()> {
    ctx.accounts.store_0.data += data;
    ctx.accounts.store_1.data += data;
    ctx.accounts.store_2.data += data;
    ctx.accounts.store_3.data += data;
    ctx.accounts.store_4.data += data;
    ctx.accounts.store_5.data += data;
    ctx.accounts.store_6.data += data;
    ctx.accounts.store_7.data += data;
    ctx.accounts.store_8.data += data;
    ctx.accounts.store_9.data += data;
    ctx.accounts.store_10.data += data;
    ctx.accounts.store_11.data += data;
    ctx.accounts.store_12.data += data;
    ctx.accounts.store_13.data += data;
    ctx.accounts.store_14.data += data;
    ctx.accounts.store_15.data += data;
    ctx.accounts.store_16.data += data;
    ctx.accounts.store_17.data += data;
    ctx.accounts.store_18.data += data;
    ctx.accounts.store_19.data += data;
    ctx.accounts.store_20.data += data;
    ctx.accounts.store_21.data += data;
    ctx.accounts.store_22.data += data;
    ctx.accounts.store_23.data += data;
    ctx.accounts.store_24.data += data;
    ctx.accounts.store_25.data += data;
    ctx.accounts.store_26.data += data;
    ctx.accounts.store_27.data += data;
    ctx.accounts.store_28.data += data;
    ctx.accounts.store_29.data += data;
    ctx.accounts.store_30.data += data;
    ctx.accounts.store_31.data += data;
    ctx.accounts.store_32.data += data;
    ctx.accounts.store_33.data += data;
    ctx.accounts.store_34.data += data;
    ctx.accounts.store_35.data += data;
    ctx.accounts.store_36.data += data;
    ctx.accounts.store_37.data += data;
    ctx.accounts.store_38.data += data;
    ctx.accounts.store_39.data += data;
    ctx.accounts.store_40.data += data;
    ctx.accounts.store_41.data += data;
    ctx.accounts.store_42.data += data;
    ctx.accounts.store_43.data += data;
    ctx.accounts.store_44.data += data;
    ctx.accounts.store_45.data += data;
    ctx.accounts.store_46.data += data;
    ctx.accounts.store_47.data += data;
    ctx.accounts.store_48.data += data;
    ctx.accounts.store_49.data += data;
    ctx.accounts.store_50.data += data;
    ctx.accounts.store_51.data += data;
    ctx.accounts.store_52.data += data;
    ctx.accounts.store_53.data += data;
    ctx.accounts.store_54.data += data;
    ctx.accounts.store_55.data += data;
    ctx.accounts.store_56.data += data;
    ctx.accounts.store_57.data += data;
    ctx.accounts.store_58.data += data;
    ctx.accounts.store_59.data += data;

    msg!("new data: {}",  ctx.accounts.store_0.data);
    Ok(())
}
