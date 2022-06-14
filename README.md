# jira-to-md-todo
Tiny app that synchronizes Jira tickets into a markdown file as to-dos.
![Demo](demo.png)
### Basics
Uses the Jira REST API and cookie auth login endpoint. Due to this, it will only work with Jira-Server.
[JQL](https://www.atlassian.com/software/jira/guides/expand-jira/jql#jql-syntax) is used to query the serch api.
The markdown file is overwritten each time.
### Settings
Self explaining. If password is not set it will ask for it.
### Missing
- There is (currently) no synchronization back to Jira.
- Paging is not implemented so the maximum is 50 todos.
