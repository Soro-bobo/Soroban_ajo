use serde_json::json;

use crate::errors::{AppError, AppResult};

const BRAND_COLOR: &str = "#059669"; // emerald-600, matches the frontend's primary accent

pub struct EmailService;

impl EmailService {
    // `html` embeds the plaintext verification/reset link, and `body` on the error
    // path is Resend's raw response — neither may be logged, or log access becomes
    // an account-takeover primitive. Only non-sensitive fields are traced.
    #[tracing::instrument(skip(api_key, html, from, subject), fields(to = %to))]
    async fn send(
        api_key: &str,
        from: &str,
        to: &str,
        subject: &str,
        html: String,
    ) -> AppResult<()> {
        let client = reqwest::Client::new();

        let response = client
            .post("https://api.resend.com/emails")
            .bearer_auth(api_key)
            .json(&json!({
                "from": from,
                "to": [to],
                "subject": subject,
                "html": html,
            }))
            .send()
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Resend request failed: {e}")))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            tracing::error!(status = %status, body = %body, "Resend API error");
            return Err(AppError::Internal(anyhow::anyhow!(
                "Resend API returned {status}"
            )));
        }

        Ok(())
    }

    pub async fn send_verification_email(
        api_key: &str,
        from: &str,
        frontend_url: &str,
        to: &str,
        display_name: &str,
        token: &str,
    ) -> AppResult<()> {
        let link = format!("{frontend_url}/verify-email?token={token}");
        let html = layout(
            "Verify your email",
            &format!(
                r#"<p>Hi {display_name},</p>
                <p>Welcome to Ajo Platform. Confirm your email address to unlock creating and joining savings groups.</p>
                <p style="text-align:center;margin:32px 0;">
                    <a href="{link}" style="{BUTTON_STYLE}">Verify email</a>
                </p>
                <p style="color:#6b7280;font-size:13px;">This link expires in 24 hours. If you didn't create an account, you can ignore this email.</p>"#
            ),
        );
        Self::send(api_key, from, to, "Verify your Ajo Platform email", html).await
    }

    pub async fn send_password_reset_email(
        api_key: &str,
        from: &str,
        frontend_url: &str,
        to: &str,
        display_name: &str,
        token: &str,
    ) -> AppResult<()> {
        let link = format!("{frontend_url}/reset-password?token={token}");
        let html = layout(
            "Reset your password",
            &format!(
                r#"<p>Hi {display_name},</p>
                <p>We received a request to reset your Ajo Platform password. Click below to choose a new one.</p>
                <p style="text-align:center;margin:32px 0;">
                    <a href="{link}" style="{BUTTON_STYLE}">Reset password</a>
                </p>
                <p style="color:#6b7280;font-size:13px;">This link expires in 30 minutes. If you didn't request this, you can safely ignore this email — your password won't change.</p>"#
            ),
        );
        Self::send(api_key, from, to, "Reset your Ajo Platform password", html).await
    }

    pub async fn send_payout_reminder_email(
        api_key: &str,
        from: &str,
        to: &str,
        display_name: &str,
        group_name: &str,
        scheduled_date: &str,
        amount: &str,
    ) -> AppResult<()> {
        let html = layout(
            "Your payout is coming up",
            &format!(
                r#"<p>Hi {display_name},</p>
                <p>You're next in line for a payout in <strong>{group_name}</strong>, scheduled for {scheduled_date}.</p>
                <p style="font-size:18px;font-weight:700;margin:20px 0;">{amount} XLM</p>
                <p style="color:#6b7280;font-size:13px;">Make sure all members are caught up on their contributions so the payout can go out on time.</p>"#
            ),
        );
        Self::send(
            api_key,
            from,
            to,
            &format!("Your {group_name} payout is coming up"),
            html,
        )
        .await
    }

    pub async fn send_payout_received_email(
        api_key: &str,
        from: &str,
        to: &str,
        display_name: &str,
        group_name: &str,
        amount: &str,
    ) -> AppResult<()> {
        let html = layout(
            "You've been paid out",
            &format!(
                r#"<p>Hi {display_name},</p>
                <p>Your payout for <strong>{group_name}</strong> has been recorded as sent.</p>
                <p style="font-size:18px;font-weight:700;margin:20px 0;">{amount} XLM</p>
                <p style="color:#6b7280;font-size:13px;">Thanks for saving with Ajo Platform.</p>"#
            ),
        );
        Self::send(
            api_key,
            from,
            to,
            &format!("You've received your {group_name} payout"),
            html,
        )
        .await
    }
}

const BUTTON_STYLE: &str = "display:inline-block;background:#059669;color:#ffffff;text-decoration:none;font-weight:600;padding:12px 28px;border-radius:8px;font-size:14px;";

fn layout(heading: &str, body_html: &str) -> String {
    format!(
        r#"<!doctype html>
<html>
<body style="margin:0;padding:0;background:#f3f4f6;font-family:-apple-system,BlinkMacSystemFont,'Segoe UI',sans-serif;">
    <table role="presentation" width="100%" cellpadding="0" cellspacing="0" style="padding:32px 16px;">
        <tr><td align="center">
            <table role="presentation" width="480" cellpadding="0" cellspacing="0" style="background:#ffffff;border-radius:16px;overflow:hidden;border:1px solid #e5e7eb;">
                <tr><td style="background:{BRAND_COLOR};padding:24px 32px;">
                    <span style="color:#ffffff;font-size:18px;font-weight:700;">Ajo Platform</span>
                </td></tr>
                <tr><td style="padding:32px;color:#111827;font-size:14px;line-height:1.6;">
                    <h1 style="font-size:20px;margin:0 0 16px;">{heading}</h1>
                    {body_html}
                </td></tr>
                <tr><td style="padding:20px 32px;background:#f9fafb;color:#9ca3af;font-size:12px;">
                    Ajo Platform — decentralized savings circles on Stellar
                </td></tr>
            </table>
        </td></tr>
    </table>
</body>
</html>"#
    )
}
