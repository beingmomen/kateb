# Security Policy | سياسة الأمان

## Reporting a Vulnerability | الإبلاغ عن ثغرة أمنية

If you discover a security vulnerability in Kateb, please report it responsibly.

**Do NOT open a public GitHub issue for security vulnerabilities.**

Instead, please email: **beingmomen@gmail.com**

Include:
- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if any)

You will receive a response within 48 hours acknowledging your report.

## Supported Versions | الإصدارات المدعومة

| Version | Supported |
|---------|-----------|
| 1.x     | Yes       |
| < 1.0   | No        |

## Security Best Practices | ممارسات الأمان

- Kateb processes audio locally on your device
- No audio data is sent to external servers (unless AI refinement is enabled)
- AI API keys are stored locally in SQLite and never transmitted except to the configured AI provider
- Always keep your API keys private and never share them
