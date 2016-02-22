# Call Notify  

[![Build Status](https://travis-ci.org/asuivelentine/call_notify.svg?branch=network)](https://travis-ci.org/asuivelentine/call_notify) [![Coverage Status](https://coveralls.io/repos/github/asuivelentine/call_notify/badge.svg?branch=network)](https://coveralls.io/github/asuivelentine/call_notify?branch=network) 
---

Share Notifications about incomming calls between your PC and android phone.

---

####development started

---

#### Protocol

---

	All messages will be send with TCP
	The payload is in Json format containing:
		- Version
		- Message type
		- Caller number
		- Datetime

#### Dependencies 

---

The associative android app

#### requirements

---

- notify the user about incomming calls
- including some imformation about the caller
- store the call history
