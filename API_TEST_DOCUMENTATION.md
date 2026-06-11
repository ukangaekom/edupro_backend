# EduPro Backend API Test Documentation

## Overview
This document describes the currently active API endpoints in `edupro_backend`, the request and response formats, authentication behavior, and frontend testing notes.

> The backend listens on `0.0.0.0:8000` by default.

## Important runtime requirements
- `DATABASE_URL` must be set in environment.
- `JWT_SECRET` must be set in environment.
- The server is started from `src/main.rs`.
- The application uses secure HTTP-only cookies for auth.

## Authentication Behavior
- Successful login sets a cookie named `auth`.
- The cookie is created with:
  - `httpOnly: true`
  - `secure: true`
  - `sameSite: Strict`
  - `path: /`
  - expires after `15` minutes
- Frontend code must include credentials when calling protected endpoints.

### Frontend fetch pattern
```js
fetch('https://your-backend/api/user/account', {
  method: 'GET',
  credentials: 'include',
})
```

> Note: Because the cookie is `secure`, local development on plain `http://localhost:8000` may not receive the cookie. Use HTTPS or adjust the cookie policy in the backend if needed for local tests.

## Active API endpoints
The active routes are mounted in `src/main.rs` as of current source:
- `POST /api/user/login`
- `POST /api/user/register`
- `GET /api/user/account`
- `POST /api/user/account/set`
- `POST /api/user/{exam_id}/analytics`
- `POST /api/organization/login`
- `POST /api/organization/register`

### Public vs protected
- Public endpoints:
  - `POST /api/user/login`
  - `POST /api/user/register`
  - `POST /api/organization/login`
  - `POST /api/organization/register`
- Protected endpoints require a valid `auth` cookie:
  - `GET /api/user/account`
  - `POST /api/user/account/set`
  - `POST /api/user/{exam_id}/analytics`

## Endpoint details

### 1) Register user
- Path: `POST /api/user/register`
- Purpose: create a new user account
- Authentication: none
- Request body: JSON
```json
{
  "firstname": "Jane",
  "lastname": "Doe",
  "email": "jane@example.com",
  "username": "janedoe",
  "pwd": "Secret123"
}
```
- Response body: JSON
```json
{
  "result": {
    "success": true
  }
}
```
- Notes:
  - The backend stores `password_hash` and returns a simple success flag.
  - On failure, the same JSON format returns `success: false`.

### 2) Login user
- Path: `POST /api/user/login`
- Purpose: authenticate a user and set auth cookie
- Authentication: none
- Request body: JSON
```json
{
  "email": "jane@example.com",
  "pwd": "Secret123"
}
```
- Response body: JSON
```json
{
  "result": {
    "success": true
  }
}
```
- Notes:
  - If login succeeds, a secure cookie `auth` is added to the response.
  - If login fails, the endpoint still returns `success: false`.
  - The frontend must send `credentials: 'include'` when calling protected API afterward.

### 3) Register organization
- Path: `POST /api/organization/register`
- Purpose: create a new organization account
- Authentication: none
- Request body: JSON
```json
{
  "organization": "Acme Academy",
  "contact_email": "admin@acme.edu",
  "contact_phone": "+2348012345678",
  "pwd": "OrgSecret123"
}
```
- Response body: JSON
```json
{
  "result": {
    "success": true
  }
}
```
- Notes:
  - Successful registration returns a simple success flag.
  - Failure returns `success: false`.

### 4) Login organization
- Path: `POST /api/organization/login`
- Purpose: authenticate an organization and set auth cookie
- Authentication: none
- Request body: JSON
```json
{
  "email": "admin@acme.edu",
  "pwd": "OrgSecret123"
}
```
- Response body: JSON
```json
{
  "result": {
    "success": true
  }
}
```
- Notes:
  - Uses the same cookie-based auth scheme as user login.
  - Frontend must use `credentials: 'include'` for protected requests.

### 5) Get user account details
- Path: `GET /api/user/account`
- Purpose: fetch the authenticated user's account info
- Authentication: required via `auth` cookie
- Request body: none
- Response body: JSON
```json
{
  "firstname": "Ekomabasi",
  "lastname": "Ukanga",
  "email": "ekomabasiuk@gmail.com",
  "username": "ekomzy",
  "total_xps": 1000,
  "rank": 1,
  "total_exams_taken": 10,
  "total_practices_taken": 10
}
```
- Notes:
  - Currently returns hardcoded static account details in the source.
  - No request payload is accepted, even though the route name is `account/set` for the other endpoint.

### 6) Set user account details
- Path: `POST /api/user/account/set`
- Purpose: currently returns user account details
- Authentication: required via `auth` cookie
- Request body: none
- Response body: same as `GET /api/user/account`
- Notes:
  - The current implementation does not accept or store a request body.
  - It returns the same static account details as the GET endpoint.

### 7) Get exam analytics for user
- Path: `POST /api/user/{exam_id}/analytics`
- Purpose: fetch analytics for a completed exam
- Authentication: required via `auth` cookie
- URL parameter:
  - `exam_id` = integer exam identifier
- Request body: none
- Response body: JSON
```json
{
  "exam_id": 12345,
  "exam_name": "JAMB",
  "exam_date": "26th May, 2026",
  "total_score": 360,
  "percent_score": 90.0,
  "subject_analytics": [
    {
      "exam_id": 12345,
      "subject_id": 0,
      "subject_name": "Mathematics",
      "score": 90,
      "total_questions": 50,
      "percent_score": 90.0
    },
    {
      "exam_id": 12345,
      "subject_id": 1,
      "subject_name": "English",
      "score": 90,
      "total_questions": 50,
      "percent_score": 90.0
    }
  ]
}
```
- Notes:
  - The returned values are placeholders from the source code.
  - The endpoint currently ignores the request body and returns static analytics values.

## API request examples
### Login and then fetch account
```js
await fetch('https://localhost:8000/api/user/login', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  credentials: 'include',
  body: JSON.stringify({ email: 'jane@example.com', pwd: 'Secret123' })
});

const accountRes = await fetch('https://localhost:8000/api/user/account', {
  method: 'GET',
  credentials: 'include',
});
const accountData = await accountRes.json();
```

### Register an organization
```js
await fetch('https://localhost:8000/api/organization/register', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({
    organization: 'Acme Academy',
    contact_email: 'admin@acme.edu',
    contact_phone: '+2348012345678',
    pwd: 'OrgSecret123'
  })
});
```

## Notes on status codes and errors
- The current handlers mostly return JSON with `result.success`.
- Authentication failures on protected endpoints return `401 Unauthorized`.
- Missing routes will fall through to the fallback error handler.
- Login/register endpoints typically return `200` and embed success/failure in JSON.

## Source-level testing guidance
For frontend test coverage, the following scenarios should be validated:

1. `POST /api/user/register`
   - valid payload -> `success: true`
   - invalid email or duplicate user -> `success: false`

2. `POST /api/user/login`
   - valid credentials -> cookie is set and `success: true`
   - invalid credentials -> `success: false`

3. Protected endpoint access
   - call `GET /api/user/account` with valid cookie -> returns account object
   - call `GET /api/user/account` without cookie -> returns `401`

4. Organization auth flow
   - `POST /api/organization/login` with correct credentials -> cookie set
   - `POST /api/organization/login` with wrong password -> `success: false`

5. Analytics endpoint
   - `POST /api/user/123/analytics` -> returns analytics JSON
   - ensure URL path param is accepted and protected by auth

## Non-mounted routes in source code
The following route files exist in the repository, but their endpoints are not merged into the live router in `src/main.rs`:

- `src/app/user/route_exams.rs`
- `src/app/user/route_practice.rs`
- `src/app/user/route_leaderboard.rs`
- `src/app/user/route_payment.rs`
- `src/app/organization/route_account.rs`
- `src/app/organization/route_payment.rs`
- `src/app/organization/route_questions.rs`
- `src/app/organization/route_exams.rs`

These are currently not available to frontend tests until they are merged into the main router and implemented.

## Data model references
### User request payloads
- `UserRegisterPayload`
  - `firstname`, `lastname`, `email`, `username`, `pwd`
- `LoginPayload`
  - `email`, `pwd`

### Organization request payloads
- `OrganizationRegisterPayload`
  - `organization`, `contact_email`, `contact_phone`, `pwd`
- `LoginPayload`
  - `email`, `pwd`

### Exam request payloads
- `answer_question`
  - `id`
- `set_exams`
  - `organization_id`, `exam_name`, `start_date`, `end_date`, `total_subjects`, `question_each`
- `submit_exams`
  - `session_id`

### Current response shapes
- `user_account_details`
- `user_exam_analytics`
- `organization_leaderboard`

## Practical frontend testing tips
- Use native browser cookie support with `credentials: 'include'`.
- Confirm the `auth` cookie is created after login.
- For local development, be aware of `secure` cookie restrictions.
- When testing protected endpoints, clear cookies first to confirm `401` behavior.
- Since account details are currently hardcoded, tests should validate shape, not dynamic values.

## Summary
This repo is currently ready for frontend tests around user/org register/login and protected user account/analytics calls. Additional route files exist, but they are either unimplemented or not mounted, so they should not be included in end-to-end frontend tests until the backend merges and completes them.
