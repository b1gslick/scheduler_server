# registration
POST {{host}}/{{version}}/registration
{"email": "{{email_2}}","password": "AbcD1x!#"}

HTTP 201

# login
POST {{host}}/{{version}}/login
{"email": "{{email_2}}", "password": "AbcD1x!#"}

HTTP 200

[Captures]
token: jsonpath "$.token"

# miss_one_field
POST {{host}}/{{version}}/activity
Authorization: {{token}}

{
    "content": "{{content}}",
    "time": {{time}}
}

HTTP 422

[Asserts]
body contains "missing field `title`"
