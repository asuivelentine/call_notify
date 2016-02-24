# Call Notify  

[![Build Status](https://travis-ci.org/asuivelentine/call_notify.svg?branch=master)](https://travis-ci.org/asuivelentine/call_notify) [![Coverage Status](https://coveralls.io/repos/github/asuivelentine/call_notify/badge.svg?branch=master)](https://coveralls.io/github/asuivelentine/call_notify?branch=master) 
---

Share Notifications about incomming calls between your PC and android phone.

---

####development started

---

#### Protocol

---

	All messages will be send with TCP
	Every package is closed by \n
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

- notify the user about new nofitications
- include notification information 
- modules can be added 
