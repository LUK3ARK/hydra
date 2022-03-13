use crate::MembershipModel;
use anchor_lang::prelude::*;

use crate::state::{Fanout, FanoutMembershipVoucher};
use crate::utils::validation::*;

use crate::utils::logic::distribution::{distribute_mint, distribute_native};
use anchor_spl::token::{Mint, Token};

#[derive(Accounts)]
#[instruction(distribute_for_mint: bool)]
pub struct DistributeWalletMember<'info> {
    pub payer: Signer<'info>,
    #[account(mut)]
    pub member: UncheckedAccount<'info>,
    #[account(
    mut,
    seeds = [b"fanout-membership", fanout.key().as_ref(), member.key().as_ref()],
    constraint = membership_voucher.membership_key == member.key(),
    bump = membership_voucher.bump_seed,
    )]
    pub membership_voucher: Box<Account<'info, FanoutMembershipVoucher>>,
    #[account(
    mut,
    seeds = [b"fanout-config", fanout.name.as_bytes()],
    bump = fanout.bump_seed,
    )]
    pub fanout: Account<'info, Fanout>,
    #[account(mut)]
    pub holding_account: UncheckedAccount<'info>,
    #[account(mut)]
    pub fanout_for_mint: UncheckedAccount<'info>,
    #[account(mut)]
    pub fanout_for_mint_membership_voucher: UncheckedAccount<'info>,
    pub fanout_mint: Account<'info, Mint>,
    #[account(mut)]
    pub fanout_mint_member_token_account: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
    pub payer_token_account: Option<UncheckedAccount<'info>>,
}

pub fn distribute_for_wallet(
    ctx: Context<DistributeWalletMember>,
    distribute_for_mint: bool,
) -> ProgramResult {
    let fanout = &mut ctx.accounts.fanout;
    let fanout_info = fanout.to_account_info();
    let membership_voucher = &mut ctx.accounts.membership_voucher;
    let membership_voucher_info = membership_voucher.to_account_info();
    let member = &mut ctx.accounts.member;
    assert_owned_by(&fanout_info, &crate::ID)?;
    assert_owned_by(&membership_voucher_info, &crate::ID)?;
    assert_owned_by(&member.to_account_info(), &System::id())?;
    assert_membership_model(fanout, MembershipModel::Wallet)?;
    assert_shares_distributed(fanout)?;

    let payer_rewards = calculate_payer_rewards(fanout.total_inflow, fanout.payer_reward_basis_points);
    if payer_rewards > 0 {
        if distribute_for_mint
    }
    if distribute_for_mint {
        if payer_rewards > 0 {
            if Some(payer_ata) in payer_token_account {
                let payer_account_ata = parse_token_account(payer_ata, payer.key)?;

                transfer_from_mint_holding(
                    ctx.accounts.fanout,
                    ctx.accounts.fanout,
                    ctx.accounts.token_program.to_owned(),
                    ctx.accounts.payer.to_account_info(),
                    payer_account_ata,
                    payer_rewards,
            }
            return Err(ErrorCode::PayerATANotSupplied.into());
        }

        let membership_key = &ctx.accounts.member.key().clone();
        let member = ctx.accounts.member.to_owned();
        distribute_mint(
            ctx.accounts.fanout_mint.to_owned(),
            &mut ctx.accounts.fanout_for_mint,
            &mut ctx.accounts.fanout_for_mint_membership_voucher,
            &mut ctx.accounts.fanout_mint_member_token_account,
            &mut ctx.accounts.holding_account,
            &mut ctx.accounts.fanout,
            &mut ctx.accounts.membership_voucher,
            ctx.accounts.rent.to_owned(),
            ctx.accounts.system_program.to_owned(),
            ctx.accounts.token_program.to_owned(),
            ctx.accounts.payer.to_account_info(),
            member,
            membership_key,
        )?;
    } else {
        if payer_rewards > 0 {
            transfer_native(
                ctx.accounts.holding_account,
                ctx.accounts.payer.to_account_info(),
                ctx.accounts.current_snapshot,
                payer_rewards,
            )?;
        }
        distribute_native(
            &mut ctx.accounts.holding_account,
            &mut ctx.accounts.fanout,
            &mut ctx.accounts.membership_voucher,
            ctx.accounts.member.to_owned(),
            ctx.accounts.rent.to_owned(),
        )?;
    }
    Ok(())
}
