# registration
POST {{host}}/{{version}}/registration
{"email": "{{email_2}}","password": "AbcD1x!#"}

HTTP 200

# login
POST {{host}}/{{version}}/login
{"email": "{{email_2}}", "password": "AbcD1x!#"}

HTTP 200

[Captures]
token: regex "\"([^\"]*)\""

# miss_one_field
POST {{host}}/{{version}}/activity

{
    "content": "{{content}}",
    "time": {{time}}
}

HTTP 422

[Asserts]
body == "missing field `title`"
