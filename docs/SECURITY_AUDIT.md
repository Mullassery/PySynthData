# Security Vulnerability Audit - pysynthdata

**Audit Date:** 2026-10-27
**Repository:** /Users/georgimullassery/Desktop/Projects/pysynthdata
**Total Findings:** 2302

## Summary
- **Critical:** 2193
- **High:** 25
- **Medium:** 84

## Findings

### 1. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/_plotly_utils/basevalidators.py:2091
- **Description:** eval()
- **Evidence (Line):** `validator_class = eval(validator_classname)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/_pytest/_code/code.py:164
- **Description:** eval()
- **Evidence (Line):** `def eval(self, code, **vars):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 3. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/_pytest/_code/code.py:173
- **Description:** eval()
- **Evidence (Line):** `return eval(code, self.f_globals, f_locals)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 4. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/_pytest/_code/code.py:326
- **Description:** eval()
- **Evidence (Line):** `# however via `exec(...)` / `eval(...)` they can be other types`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 5. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/_pytest/_code/code.py:326
- **Description:** exec()
- **Evidence (Line):** `# however via `exec(...)` / `eval(...)` they can be other types`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 6. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/_pytest/_py/path.py:1153
- **Description:** exec()
- **Evidence (Line):** `exec(f.read(), mod.__dict__)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 7. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/_pytest/_py/path.py:1159
- **Description:** exec()
- **Evidence (Line):** `def sysexec(self, *argv: os.PathLike[str], **popen_opts: Any) -> str:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 8. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/_pytest/assertion/rewrite.py:188
- **Description:** exec()
- **Evidence (Line):** `exec(co, module.__dict__)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 9. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/_pytest/assertion/rewrite.py:393
- **Description:** marshal load
- **Evidence (Line):** `co = marshal.load(fp)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 10. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/_pytest/capture.py:1112
- **Description:** os.system()
- **Evidence (Line):** `os.system('echo "hello"')`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 11. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/_pytest/capture.py:1140
- **Description:** os.system()
- **Evidence (Line):** `os.system('echo "hello"')`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 12. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/_pytest/mark/__init__.py:66
- **Description:** eval()
- **Evidence (Line):** `def test_eval(test_input, expected):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 13. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/_pytest/mark/__init__.py:67
- **Description:** eval()
- **Evidence (Line):** `assert eval(test_input) == expected`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 14. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/_pytest/mark/expression.py:295
- **Description:** eval()
- **Evidence (Line):** `"""Adapts a matcher function to a locals mapping as required by eval()."""`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 15. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/_pytest/mark/expression.py:353
- **Description:** eval()
- **Evidence (Line):** `return bool(eval(self._code, {"__builtins__": {}}, MatcherAdapter(matcher)))`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 16. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/_pytest/pytester.py:296
- **Description:** eval()
- **Evidence (Line):** `if eval(check, backlocals, call.__dict__):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 17. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/_pytest/skipping.py:92
- **Description:** eval()
- **Evidence (Line):** `If an old-style string condition is given, it is eval()'d, otherwise the`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 18. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/_pytest/skipping.py:119
- **Description:** eval()
- **Evidence (Line):** `result = eval(condition_code, globals_)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 19. [Critical] Hardcoded Secret
- **File:** .pytest_venv/lib/python3.11/site-packages/botocore/credentials.py:1189
- **Description:** Secret Key Assignment detected
- **Evidence (Line):** `SECRET_KEY = 'AWS_SECRET_ACCESS_KEY'`
- **Exploit Scenario:** Attacker could extract credentials and gain unauthorized access
- **Fix Recommendation:** Move secrets to environment variables or secret manager, rotate compromised credentials
- **Confidence:** High

### 20. [Critical] Hardcoded Secret
- **File:** .pytest_venv/lib/python3.11/site-packages/botocore/credentials.py:1368
- **Description:** Secret Key Assignment detected
- **Evidence (Line):** `SECRET_KEY = 'aws_secret_access_key'`
- **Exploit Scenario:** Attacker could extract credentials and gain unauthorized access
- **Fix Recommendation:** Move secrets to environment variables or secret manager, rotate compromised credentials
- **Confidence:** High

### 21. [Critical] Hardcoded Secret
- **File:** .pytest_venv/lib/python3.11/site-packages/botocore/credentials.py:1426
- **Description:** Secret Key Assignment detected
- **Evidence (Line):** `SECRET_KEY = 'aws_secret_access_key'`
- **Exploit Scenario:** Attacker could extract credentials and gain unauthorized access
- **Fix Recommendation:** Move secrets to environment variables or secret manager, rotate compromised credentials
- **Confidence:** High

### 22. [Critical] Hardcoded Secret
- **File:** .pytest_venv/lib/python3.11/site-packages/botocore/credentials.py:1496
- **Description:** Secret Key Assignment detected
- **Evidence (Line):** `SECRET_KEY = 'aws_secret_access_key'`
- **Exploit Scenario:** Attacker could extract credentials and gain unauthorized access
- **Fix Recommendation:** Move secrets to environment variables or secret manager, rotate compromised credentials
- **Confidence:** High

### 23. [Critical] Hardcoded Secret
- **File:** .pytest_venv/lib/python3.11/site-packages/botocore/credentials.py:2073
- **Description:** Token Assignment detected
- **Evidence (Line):** `ENV_VAR_AUTH_TOKEN = 'AWS_CONTAINER_AUTHORIZATION_TOKEN'`
- **Exploit Scenario:** Attacker could extract credentials and gain unauthorized access
- **Fix Recommendation:** Move secrets to environment variables or secret manager, rotate compromised credentials
- **Confidence:** High

### 24. [Critical] Hardcoded Secret
- **File:** .pytest_venv/lib/python3.11/site-packages/botocore/data/iam/2010-05-08/examples-1.json:130
- **Description:** AWS Access Key detected
- **Evidence (Line):** `"AccessKeyId": "AKIAIOSFODNN7EXAMPLE",`
- **Exploit Scenario:** Attacker could extract credentials and gain unauthorized access
- **Fix Recommendation:** Move secrets to environment variables or secret manager, rotate compromised credentials
- **Confidence:** High

### 25. [Critical] Hardcoded Secret
- **File:** .pytest_venv/lib/python3.11/site-packages/botocore/data/iam/2010-05-08/examples-1.json:280
- **Description:** AWS Access Key detected
- **Evidence (Line):** `"RoleId": "AKIAIOSFODNN7EXAMPLE",`
- **Exploit Scenario:** Attacker could extract credentials and gain unauthorized access
- **Fix Recommendation:** Move secrets to environment variables or secret manager, rotate compromised credentials
- **Confidence:** High

### 26. [Critical] Hardcoded Secret
- **File:** .pytest_venv/lib/python3.11/site-packages/botocore/data/iam/2010-05-08/examples-1.json:305
- **Description:** AWS Access Key detected
- **Evidence (Line):** `"UserId": "AKIAIOSFODNN7EXAMPLE",`
- **Exploit Scenario:** Attacker could extract credentials and gain unauthorized access
- **Fix Recommendation:** Move secrets to environment variables or secret manager, rotate compromised credentials
- **Confidence:** High

### 27. [Critical] Hardcoded Secret
- **File:** .pytest_venv/lib/python3.11/site-packages/botocore/data/iam/2010-05-08/examples-1.json:847
- **Description:** AWS Access Key detected
- **Evidence (Line):** `"UserId": "AKIAIOSFODNN7EXAMPLE",`
- **Exploit Scenario:** Attacker could extract credentials and gain unauthorized access
- **Fix Recommendation:** Move secrets to environment variables or secret manager, rotate compromised credentials
- **Confidence:** High

### 28. [Critical] Hardcoded Secret
- **File:** .pytest_venv/lib/python3.11/site-packages/botocore/data/iam/2010-05-08/examples-1.json:870
- **Description:** AWS Access Key detected
- **Evidence (Line):** `"AccessKeyId": "AKIA111111111EXAMPLE",`
- **Exploit Scenario:** Attacker could extract credentials and gain unauthorized access
- **Fix Recommendation:** Move secrets to environment variables or secret manager, rotate compromised credentials
- **Confidence:** High

### 29. [Critical] Hardcoded Secret
- **File:** .pytest_venv/lib/python3.11/site-packages/botocore/data/iam/2010-05-08/examples-1.json:876
- **Description:** AWS Access Key detected
- **Evidence (Line):** `"AccessKeyId": "AKIA222222222EXAMPLE",`
- **Exploit Scenario:** Attacker could extract credentials and gain unauthorized access
- **Fix Recommendation:** Move secrets to environment variables or secret manager, rotate compromised credentials
- **Confidence:** High

### 30. [Critical] Hardcoded Secret
- **File:** .pytest_venv/lib/python3.11/site-packages/botocore/data/iam/2010-05-08/examples-1.json:1403
- **Description:** AWS Access Key detected
- **Evidence (Line):** `"AccessKeyId": "AKIAIOSFODNN7EXAMPLE",`
- **Exploit Scenario:** Attacker could extract credentials and gain unauthorized access
- **Fix Recommendation:** Move secrets to environment variables or secret manager, rotate compromised credentials
- **Confidence:** High

### 31. [Critical] Hardcoded Secret
- **File:** .pytest_venv/lib/python3.11/site-packages/botocore/data/iam/2010-05-08/examples-1.json:1526
- **Description:** Private Key detected
- **Evidence (Line):** `"PrivateKey": "-----BEGIN DSA PRIVATE KEY-----<a very long private key string>-----END DSA PRIVATE K`
- **Exploit Scenario:** Attacker could extract credentials and gain unauthorized access
- **Fix Recommendation:** Move secrets to environment variables or secret manager, rotate compromised credentials
- **Confidence:** High

### 32. [Critical] Hardcoded Secret
- **File:** .pytest_venv/lib/python3.11/site-packages/botocore/data/iam/2010-05-08/examples-1.json:1430
- **Description:** Password Assignment detected
- **Evidence (Line):** `"description": "The following command sets the password policy to require a minimum length of eight `
- **Exploit Scenario:** Attacker could extract credentials and gain unauthorized access
- **Fix Recommendation:** Move secrets to environment variables or secret manager, rotate compromised credentials
- **Confidence:** High

### 33. [Critical] Hardcoded Secret
- **File:** .pytest_venv/lib/python3.11/site-packages/botocore/data/sts/2011-06-15/examples-1.json:36
- **Description:** AWS Access Key detected
- **Evidence (Line):** `"AccessKeyId": "AKIAIOSFODNN7EXAMPLE",`
- **Exploit Scenario:** Attacker could extract credentials and gain unauthorized access
- **Fix Recommendation:** Move secrets to environment variables or secret manager, rotate compromised credentials
- **Confidence:** High

### 34. [Critical] Hardcoded Secret
- **File:** .pytest_venv/lib/python3.11/site-packages/botocore/data/sts/2011-06-15/examples-1.json:108
- **Description:** AWS Access Key detected
- **Evidence (Line):** `"AccessKeyId": "AKIAIOSFODNN7EXAMPLE",`
- **Exploit Scenario:** Attacker could extract credentials and gain unauthorized access
- **Fix Recommendation:** Move secrets to environment variables or secret manager, rotate compromised credentials
- **Confidence:** High

### 35. [Critical] Hardcoded Secret
- **File:** .pytest_venv/lib/python3.11/site-packages/botocore/data/sts/2011-06-15/examples-1.json:154
- **Description:** AWS Access Key detected
- **Evidence (Line):** `"UserId": "AKIAI44QH8DHBEXAMPLE"`
- **Exploit Scenario:** Attacker could extract credentials and gain unauthorized access
- **Fix Recommendation:** Move secrets to environment variables or secret manager, rotate compromised credentials
- **Confidence:** High

### 36. [Critical] Hardcoded Secret
- **File:** .pytest_venv/lib/python3.11/site-packages/botocore/data/sts/2011-06-15/examples-1.json:172
- **Description:** AWS Access Key detected
- **Evidence (Line):** `"UserId": "AKIAI44QH8DHBEXAMPLE:my-role-session-name"`
- **Exploit Scenario:** Attacker could extract credentials and gain unauthorized access
- **Fix Recommendation:** Move secrets to environment variables or secret manager, rotate compromised credentials
- **Confidence:** High

### 37. [Critical] Hardcoded Secret
- **File:** .pytest_venv/lib/python3.11/site-packages/botocore/data/sts/2011-06-15/examples-1.json:222
- **Description:** AWS Access Key detected
- **Evidence (Line):** `"AccessKeyId": "AKIAIOSFODNN7EXAMPLE",`
- **Exploit Scenario:** Attacker could extract credentials and gain unauthorized access
- **Fix Recommendation:** Move secrets to environment variables or secret manager, rotate compromised credentials
- **Confidence:** High

### 38. [Critical] Hardcoded Secret
- **File:** .pytest_venv/lib/python3.11/site-packages/botocore/data/sts/2011-06-15/examples-1.json:253
- **Description:** AWS Access Key detected
- **Evidence (Line):** `"AccessKeyId": "AKIAIOSFODNN7EXAMPLE",`
- **Exploit Scenario:** Attacker could extract credentials and gain unauthorized access
- **Fix Recommendation:** Move secrets to environment variables or secret manager, rotate compromised credentials
- **Confidence:** High

### 39. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/botocore/vendored/six.py:735
- **Description:** exec()
- **Evidence (Line):** `exec("""exec _code_ in _globs_, _locs_""")`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 40. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/copulas/multivariate/base.py:178
- **Description:** pickle load/dumps
- **Evidence (Line):** `return pickle.load(pickle_file)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 41. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/copulas/univariate/base.py:488
- **Description:** pickle load/dumps
- **Evidence (Line):** `return pickle.load(pickle_file)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 42. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/ctgan/synthesizers/tvae.py:222
- **Description:** eval()
- **Evidence (Line):** `self.decoder.eval()`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 43. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/faker/sphinx/docstring.py:199
- **Description:** eval()
- **Evidence (Line):** `results = "\n".join([self._stringify_result(eval(command, eval_scope)) for _ in range(sample.size)])`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 44. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/faker/sphinx/validator.py:15
- **Description:** eval()
- **Evidence (Line):** `is to pass a command string to `eval()` should the string pass the validation`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 45. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/faker/sphinx/validator.py:21
- **Description:** eval()
- **Evidence (Line):** `within the scope where `eval()` will be called, {method} will be the provider`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 46. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/faker/sphinx/validator.py:22
- **Description:** eval()
- **Evidence (Line):** `method name which is also available within the `eval()` scope, and {arguments}`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 47. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/faker/sphinx/validator.py:44
- **Description:** eval()
- **Evidence (Line):** `the standard library's `collections.OrderedDict` within the `eval()` scope before`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 48. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/faker/sphinx/validator.py:45
- **Description:** eval()
- **Evidence (Line):** `passing the command string to `eval()` for execution. This can be done in code review.`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 49. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/fsspec/gui.py:312
- **Description:** eval()
- **Evidence (Line):** `return ast.literal_eval(self.kwargs.value) or {}`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 50. [Critical] Hardcoded Secret
- **File:** .pytest_venv/lib/python3.11/site-packages/fsspec/spec.py:1718
- **Description:** Password Assignment detected
- **Evidence (Line):** `>>> fs = filesystem('ftp', host='test.rebex.net', user='demo', password='password')`
- **Exploit Scenario:** Attacker could extract credentials and gain unauthorized access
- **Fix Recommendation:** Move secrets to environment variables or secret manager, rotate compromised credentials
- **Confidence:** High

### 51. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/fsspec/tests/abstract/mv.py:36
- **Description:** os.system()
- **Evidence (Line):** `os.system(f'icacls "{protected_path}" /deny Everyone:(W)')`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 52. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/fsspec/tests/abstract/mv.py:55
- **Description:** os.system()
- **Evidence (Line):** `os.system(f'icacls "{protected_path}" /remove:d Everyone')`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 53. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/functorch/einops/rearrange.py:149
- **Description:** exec()
- **Evidence (Line):** `exec(custom_rearrange_callable_code, globals(), namespace)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 54. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/jinja2/bccache.py:73
- **Description:** pickle load/dumps
- **Evidence (Line):** `checksum = pickle.load(f)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 55. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/jinja2/bccache.py:79
- **Description:** marshal load
- **Evidence (Line):** `self.code = marshal.load(f)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 56. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/jinja2/debug.py:145
- **Description:** exec()
- **Evidence (Line):** `exec(code, globals, locals)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 57. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/jinja2/environment.py:1228
- **Description:** exec()
- **Evidence (Line):** `exec(code, namespace)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 58. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/jinja2/lexer.py:663
- **Description:** eval()
- **Evidence (Line):** `value = literal_eval(value_str.replace("_", ""))`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 59. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/jinja2/nativetypes.py:40
- **Description:** eval()
- **Evidence (Line):** `return literal_eval(`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 60. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/jinja2/nodes.py:581
- **Description:** eval()
- **Evidence (Line):** `representation (objects where ``eval(repr(x)) == x`` is true).`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 61. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/_store_backends.py:196
- **Description:** pickle load/dumps
- **Evidence (Line):** `item = numpy_pickle.load(f)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 62. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/_store_backends.py:198
- **Description:** pickle load/dumps
- **Evidence (Line):** `item = numpy_pickle.load(filename, mmap_mode=mmap_mode)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 63. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/externals/loky/backend/fork_exec.py:12
- **Description:** exec()
- **Evidence (Line):** `def fork_exec(cmd, keep_fds, env=None):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 64. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/externals/loky/backend/fork_exec.py:46
- **Description:** exec()
- **Evidence (Line):** `return _posixsubprocess.fork_exec(`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 65. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/externals/loky/backend/popen_loky_posix.py:128
- **Description:** exec()
- **Evidence (Line):** `pid = fork_exec(cmd_python, self._fds, env=process_obj.env)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 66. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/externals/loky/backend/popen_loky_posix.py:174
- **Description:** pickle load/dumps
- **Evidence (Line):** `prep_data = pickle.load(from_parent)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 67. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/externals/loky/backend/popen_loky_posix.py:176
- **Description:** pickle load/dumps
- **Evidence (Line):** `process_obj = pickle.load(from_parent)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 68. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/hashing.py:231
- **Description:** pickle load/dumps
- **Evidence (Line):** `# ``pickle.loads(pickle.dumps(np.dtype('f4'))) is not`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 69. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/hashing.py:236
- **Description:** pickle load/dumps
- **Evidence (Line):** `# and ``[np.dtype('f4'), pickle.loads(pickle.dumps('f4'))]``.`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 70. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/memory.py:178
- **Description:** eval()
- **Evidence (Line):** `(namely eval(repr(memorized_instance)) works).`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 71. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/memory.py:1192
- **Description:** eval()
- **Evidence (Line):** `def eval(self, func, *args, **kwargs):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 72. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/parallel.py:1771
- **Description:** eval()
- **Evidence (Line):** `def _wait_retrieval(self):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 73. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/parallel.py:1800
- **Description:** eval()
- **Evidence (Line):** `while self._wait_retrieval():`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 74. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/test/test_func_inspect.py:366
- **Description:** exec()
- **Evidence (Line):** `exec(`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 75. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/test/test_hashing.py:350
- **Description:** pickle load/dumps
- **Evidence (Line):** `c = pickle.loads(pickle.dumps(b))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 76. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/test/test_hashing.py:370
- **Description:** pickle load/dumps
- **Evidence (Line):** `dt1_roundtripped = pickle.loads(pickle.dumps(dt1))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 77. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/test/test_hashing.py:383
- **Description:** pickle load/dumps
- **Evidence (Line):** `complex_dt1_roundtripped = pickle.loads(pickle.dumps(complex_dt1))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 78. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/test/test_memory.py:129
- **Description:** eval()
- **Evidence (Line):** `assert memory.eval(f, 1) == out`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 79. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/test/test_memory.py:400
- **Description:** eval()
- **Evidence (Line):** `a1 = eval("lambda x: x")`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 80. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/test/test_memory.py:402
- **Description:** eval()
- **Evidence (Line):** `b1 = eval("lambda x: x+1")`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 81. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/test/test_memory.py:430
- **Description:** eval()
- **Evidence (Line):** `def test_memory_eval(tmpdir):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 82. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/test/test_memory.py:434
- **Description:** eval()
- **Evidence (Line):** `m = eval("lambda x: x")`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 83. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/test/test_memory.py:166
- **Description:** exec()
- **Evidence (Line):** `exec(`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 84. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/test/test_memory.py:257
- **Description:** exec()
- **Evidence (Line):** `exec(`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 85. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/test/test_memory.py:686
- **Description:** pickle load/dumps
- **Evidence (Line):** `h = pickle.loads(pickle.dumps(g))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 86. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/test/test_memory.py:692
- **Description:** pickle load/dumps
- **Evidence (Line):** `memory2 = pickle.loads(pickle.dumps(memory))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 87. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/test/test_memory.py:697
- **Description:** pickle load/dumps
- **Evidence (Line):** `pickle.loads(pickle.dumps(memory))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 88. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/test/test_memory.py:699
- **Description:** pickle load/dumps
- **Evidence (Line):** `gp = pickle.loads(pickle.dumps(g))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 89. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/test/test_memory.py:792
- **Description:** pickle load/dumps
- **Evidence (Line):** `result2 = pickle.load(fp)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 90. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/test/test_memory.py:1425
- **Description:** pickle load/dumps
- **Evidence (Line):** `memorized_result_loads = pickle.loads(memorized_result_pickle)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 91. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/test/test_memory.py:1459
- **Description:** pickle load/dumps
- **Evidence (Line):** `memory_reloaded = pickle.loads(pickle.dumps(memory))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 92. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/test/test_memory.py:1472
- **Description:** pickle load/dumps
- **Evidence (Line):** `func_cached_reloaded = pickle.loads(pickle.dumps(func_cached))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 93. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/test/test_memory.py:1485
- **Description:** pickle load/dumps
- **Evidence (Line):** `memorized_result_reloaded = pickle.loads(pickle.dumps(memorized_result))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 94. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/test/test_memory_async.py:69
- **Description:** eval()
- **Evidence (Line):** `evaled = await memory.eval(f, 1)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 95. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/test/test_numpy_pickle.py:119
- **Description:** pickle load/dumps
- **Evidence (Line):** `_member = numpy_pickle.load(filename)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 96. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/test/test_numpy_pickle.py:159
- **Description:** pickle load/dumps
- **Evidence (Line):** `obj_ = numpy_pickle.load(filename)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 97. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/test/test_numpy_pickle.py:172
- **Description:** pickle load/dumps
- **Evidence (Line):** `obj_ = numpy_pickle.load(filename)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 98. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/test/test_numpy_pickle.py:185
- **Description:** pickle load/dumps
- **Evidence (Line):** `obj_loaded = numpy_pickle.load(filename)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 99. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/test/test_numpy_pickle.py:197
- **Description:** pickle load/dumps
- **Evidence (Line):** `arr_reloaded = numpy_pickle.load(filename)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 100. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/test/test_numpy_pickle.py:208
- **Description:** pickle load/dumps
- **Evidence (Line):** `b = numpy_pickle.load(filename, mmap_mode="r")`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 101. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/test/test_numpy_pickle.py:216
- **Description:** pickle load/dumps
- **Evidence (Line):** `obj_loaded = numpy_pickle.load(filename, mmap_mode="r")`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 102. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/test/test_numpy_pickle.py:229
- **Description:** pickle load/dumps
- **Evidence (Line):** `obj_loaded = numpy_pickle.load(filename, mmap_mode="r+")`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 103. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/test/test_numpy_pickle.py:235
- **Description:** pickle load/dumps
- **Evidence (Line):** `obj_reloaded = numpy_pickle.load(filename, mmap_mode="r")`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 104. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/test/test_numpy_pickle.py:240
- **Description:** pickle load/dumps
- **Evidence (Line):** `numpy_pickle.load(filename, mmap_mode="w+")`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 105. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/test/test_numpy_pickle.py:257
- **Description:** pickle load/dumps
- **Evidence (Line):** `a_clone, b_clone = numpy_pickle.load(filename, mmap_mode="r")`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 106. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/test/test_numpy_pickle.py:312
- **Description:** pickle load/dumps
- **Evidence (Line):** `loaded = numpy_pickle.load(filename)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 107. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/test/test_numpy_pickle.py:326
- **Description:** pickle load/dumps
- **Evidence (Line):** `b = numpy_pickle.load(filename, mmap_mode="r")`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 108. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/test/test_numpy_pickle.py:338
- **Description:** pickle load/dumps
- **Evidence (Line):** `reloaded_obj = numpy_pickle.load(this_filename, mmap_mode="r+")`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 109. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/test/test_numpy_pickle.py:392
- **Description:** pickle load/dumps
- **Evidence (Line):** `result_list = numpy_pickle.load(fname)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 110. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/test/test_numpy_pickle.py:415
- **Description:** pickle load/dumps
- **Evidence (Line):** `le_array_native_load, be_array_native_load = numpy_pickle.load(`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 111. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/test/test_numpy_pickle.py:422
- **Description:** pickle load/dumps
- **Evidence (Line):** `le_array_nonnative_load, be_array_nonnative_load = numpy_pickle.load(`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 112. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/test/test_numpy_pickle.py:437
- **Description:** pickle load/dumps
- **Evidence (Line):** `numpy_pickle.load(`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 113. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/test/test_numpy_pickle.py:465
- **Description:** pickle load/dumps
- **Evidence (Line):** `result_list = numpy_pickle.load(filename, mmap_mode=mmap_mode)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 114. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/test/test_numpy_pickle.py:529
- **Description:** pickle load/dumps
- **Evidence (Line):** `numpy_pickle.load(filename)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 115. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/test/test_numpy_pickle.py:719
- **Description:** pickle load/dumps
- **Evidence (Line):** `obj_reloaded = numpy_pickle.load(dump_filename)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 116. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/test/test_numpy_pickle.py:763
- **Description:** pickle load/dumps
- **Evidence (Line):** `obj_reloaded = numpy_pickle.load(filename_raw)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 117. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/test/test_numpy_pickle.py:794
- **Description:** pickle load/dumps
- **Evidence (Line):** `obj_reloaded = numpy_pickle.load(dump_fname)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 118. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/test/test_numpy_pickle.py:815
- **Description:** pickle load/dumps
- **Evidence (Line):** `obj_reloaded = numpy_pickle.load(f)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 119. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/test/test_numpy_pickle.py:820
- **Description:** pickle load/dumps
- **Evidence (Line):** `obj_reloaded_2 = numpy_pickle.load(f)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 120. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/test/test_numpy_pickle.py:836
- **Description:** pickle load/dumps
- **Evidence (Line):** `obj_reloaded = numpy_pickle.load(f)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 121. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/test/test_numpy_pickle.py:852
- **Description:** pickle load/dumps
- **Evidence (Line):** `obj_reloaded = numpy_pickle.load(f, mmap_mode="r+")`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 122. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/test/test_numpy_pickle.py:867
- **Description:** pickle load/dumps
- **Evidence (Line):** `numpy_pickle.load(f, mmap_mode="r+")`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 123. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/test/test_numpy_pickle.py:884
- **Description:** pickle load/dumps
- **Evidence (Line):** `numpy_pickle.load(buf, mmap_mode="r+")`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 124. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/test/test_numpy_pickle.py:1008
- **Description:** pickle load/dumps
- **Evidence (Line):** `c = numpy_pickle.load(filename)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 125. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/test/test_numpy_pickle.py:1017
- **Description:** pickle load/dumps
- **Evidence (Line):** `assert numpy_pickle.load(filename) == value`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 126. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/test/test_numpy_pickle.py:1019
- **Description:** pickle load/dumps
- **Evidence (Line):** `assert numpy_pickle.load(Path(filename)) == value`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 127. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/test/test_numpy_pickle.py:1036
- **Description:** pickle load/dumps
- **Evidence (Line):** `assert numpy_pickle.load(path) == value`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 128. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/test/test_numpy_pickle.py:1053
- **Description:** pickle load/dumps
- **Evidence (Line):** `array_reloaded = numpy_pickle.load(filename)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 129. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/test/test_numpy_pickle.py:1067
- **Description:** pickle load/dumps
- **Evidence (Line):** `array_reloaded = numpy_pickle.load(filename)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 130. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/test/test_numpy_pickle.py:1088
- **Description:** pickle load/dumps
- **Evidence (Line):** `array_reloaded = numpy_pickle.load(cf)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 131. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/test/test_numpy_pickle.py:1099
- **Description:** pickle load/dumps
- **Evidence (Line):** `array_reloaded = numpy_pickle.load(cf)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 132. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/test/test_numpy_pickle.py:1114
- **Description:** pickle load/dumps
- **Evidence (Line):** `memmaps = numpy_pickle.load(fname, mmap_mode="r")`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 133. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/test/test_numpy_pickle.py:1217
- **Description:** pickle load/dumps
- **Evidence (Line):** `assert numpy_pickle.load(fname) == data`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 134. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/test/test_numpy_pickle.py:1223
- **Description:** pickle load/dumps
- **Evidence (Line):** `assert numpy_pickle.load(fname) == data`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 135. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/test/test_numpy_pickle.py:1254
- **Description:** pickle load/dumps
- **Evidence (Line):** `memmap = numpy_pickle.load(fname, mmap_mode="r")`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 136. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/test/test_numpy_pickle.py:1270
- **Description:** pickle load/dumps
- **Evidence (Line):** `l_reloaded = numpy_pickle.load(fname, mmap_mode="r")`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 137. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/test/test_numpy_pickle.py:1293
- **Description:** pickle load/dumps
- **Evidence (Line):** `d_reloaded = numpy_pickle.load(fname, mmap_mode="r")`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 138. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/test/test_store_backends.py:33
- **Description:** pickle load/dumps
- **Evidence (Line):** `reloaded = cpickle.load(f)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 139. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/test/test_utils.py:9
- **Description:** exec()
- **Evidence (Line):** `"exec('import os')",`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 140. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/mpmath/identification.py:690
- **Description:** eval()
- **Evidence (Line):** `the formula, Python's :func:`~mpmath.eval()` may erroneously perform`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 141. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/mpmath/identification.py:777
- **Description:** eval()
- **Evidence (Line):** `constants = [(eval(p, namespace), p) for p in constants]`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 142. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/mpmath/libmp/backend.py:47
- **Description:** exec()
- **Evidence (Line):** `exec("""exec _code_ in _globs_, _locs_""")`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 143. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/mpmath/tests/extratest_gamma.py:56
- **Description:** eval()
- **Evidence (Line):** `z = eval(z)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 144. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/mpmath/tests/extratest_gamma.py:64
- **Description:** eval()
- **Evidence (Line):** `loggamma_val = eval(result)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 145. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/mpmath/tests/test_convert.py:82
- **Description:** eval()
- **Evidence (Line):** `"""Test that eval(repr(x)) == x"""`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 146. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/mpmath/tests/test_convert.py:88
- **Description:** eval()
- **Evidence (Line):** `assert eval(repr(a)) == a`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 147. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/mpmath/tests/test_fp.py:19
- **Description:** eval()
- **Evidence (Line):** `ans = str(eval(testline))`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 148. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/mpmath/tests/test_linalg.py:181
- **Description:** eval()
- **Evidence (Line):** `pytest.raises((ZeroDivisionError, ValueError), lambda: eval(i))`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 149. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/mpmath/tests/test_matrices.py:25
- **Description:** eval()
- **Evidence (Line):** `assert A6 == eval(repr(A6))`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 150. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/mpmath/tests/test_matrices.py:27
- **Description:** eval()
- **Evidence (Line):** `assert A6 == eval(repr(A6))`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 151. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/mpmath/tests/test_matrices.py:28
- **Description:** eval()
- **Evidence (Line):** `assert A6*1j == eval(repr(A6*1j))`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 152. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/mpmath/tests/test_matrices.py:55
- **Description:** exec()
- **Evidence (Line):** `To avoid syntax errors when importing this file in Python 3.5 and below, we have to use exec() - sor`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 153. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/mpmath/tests/test_matrices.py:57
- **Description:** exec()
- **Evidence (Line):** `# TODO remove exec() wrapper as soon as we drop support for Python <= 3.5`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 154. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/mpmath/tests/test_matrices.py:63
- **Description:** exec()
- **Evidence (Line):** `exec("assert A4 @ A5 == A4 * A5")`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 155. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/mpmath/tests/test_pickle.py:15
- **Description:** pickle load/dumps
- **Evidence (Line):** `obj2 = pickle.load(f)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 156. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/narwhals/_polars/expr.py:443
- **Description:** eval()
- **Evidence (Line):** `.list.eval(pl.element().str.to_titlecase())`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 157. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/networkx/algorithms/bipartite/edgelist.py:244
- **Description:** eval()
- **Evidence (Line):** `edgedata = dict(literal_eval(" ".join(d)))`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 158. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/networkx/algorithms/flow/tests/test_maxflow_large_graph.py:56
- **Description:** pickle load/dumps
- **Evidence (Line):** `G = pickle.load(f)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 159. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/networkx/algorithms/flow/tests/test_mincost.py:469
- **Description:** pickle load/dumps
- **Evidence (Line):** `G = pickle.load(f)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 160. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/networkx/algorithms/flow/tests/test_networksimplex.py:180
- **Description:** pickle load/dumps
- **Evidence (Line):** `G = pickle.load(f)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 161. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/networkx/algorithms/operators/tests/test_all.py:28
- **Description:** eval()
- **Evidence (Line):** `assert ghj.nodes[n] == eval(graph).nodes[int(node)]`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 162. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/networkx/algorithms/operators/tests/test_binary.py:23
- **Description:** eval()
- **Evidence (Line):** `assert gh.nodes[n] == eval(graph).nodes[int(node)]`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 163. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/networkx/classes/tests/test_coreviews.py:16
- **Description:** pickle load/dumps
- **Evidence (Line):** `pview = pickle.loads(pickle.dumps(view, -1))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 164. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/networkx/classes/tests/test_coreviews.py:19
- **Description:** pickle load/dumps
- **Evidence (Line):** `pview = pickle.loads(pickle.dumps(view))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 165. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/networkx/classes/tests/test_coreviews.py:73
- **Description:** pickle load/dumps
- **Evidence (Line):** `pview = pickle.loads(pickle.dumps(view, -1))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 166. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/networkx/classes/tests/test_coreviews.py:153
- **Description:** pickle load/dumps
- **Evidence (Line):** `pview = pickle.loads(pickle.dumps(view, -1))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 167. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/networkx/classes/tests/test_coreviews.py:211
- **Description:** pickle load/dumps
- **Evidence (Line):** `pview = pickle.loads(pickle.dumps(view, -1))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 168. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/networkx/classes/tests/test_graph.py:610
- **Description:** pickle load/dumps
- **Evidence (Line):** `pg = pickle.loads(pickle.dumps(G, -1))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 169. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/networkx/classes/tests/test_graph.py:612
- **Description:** pickle load/dumps
- **Evidence (Line):** `pg = pickle.loads(pickle.dumps(G))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 170. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/networkx/classes/tests/test_graphviews.py:18
- **Description:** pickle load/dumps
- **Evidence (Line):** `prv = pickle.loads(pickle.dumps(rv, -1))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 171. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/networkx/classes/tests/test_graphviews.py:65
- **Description:** pickle load/dumps
- **Evidence (Line):** `prv = pickle.loads(pickle.dumps(rv, -1))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 172. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/networkx/classes/tests/test_graphviews.py:118
- **Description:** pickle load/dumps
- **Evidence (Line):** `pdv = pickle.loads(pickle.dumps(dv, -1))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 173. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/networkx/classes/tests/test_graphviews.py:157
- **Description:** pickle load/dumps
- **Evidence (Line):** `puv = pickle.loads(pickle.dumps(uv, -1))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 174. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/networkx/classes/tests/test_graphviews.py:206
- **Description:** pickle load/dumps
- **Evidence (Line):** `H = pickle.loads(pickle.dumps(G, -1))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 175. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/networkx/classes/tests/test_reportviews.py:22
- **Description:** pickle load/dumps
- **Evidence (Line):** `pnv = pickle.loads(pickle.dumps(nv, -1))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 176. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/networkx/classes/tests/test_reportviews.py:93
- **Description:** pickle load/dumps
- **Evidence (Line):** `pnv = pickle.loads(pickle.dumps(nv, -1))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 177. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/networkx/classes/tests/test_reportviews.py:278
- **Description:** pickle load/dumps
- **Evidence (Line):** `pev = pickle.loads(pickle.dumps(ev, -1))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 178. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/networkx/classes/tests/test_reportviews.py:568
- **Description:** pickle load/dumps
- **Evidence (Line):** `pev = pickle.loads(pickle.dumps(ev, -1))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 179. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/networkx/classes/tests/test_reportviews.py:1023
- **Description:** pickle load/dumps
- **Evidence (Line):** `pdeg = pickle.loads(pickle.dumps(deg, -1))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 180. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/networkx/classes/tests/test_reportviews.py:1412
- **Description:** pickle load/dumps
- **Evidence (Line):** `pickle.loads(pickle.dumps(G, -1))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 181. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/networkx/readwrite/edgelist.py:275
- **Description:** eval()
- **Evidence (Line):** `edgedata = dict(literal_eval(edgedata_str.strip()))`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 182. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/networkx/readwrite/gml.py:107
- **Description:** eval()
- **Evidence (Line):** `return literal_eval(rep)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 183. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/networkx/readwrite/multiline_adjlist.py:295
- **Description:** eval()
- **Evidence (Line):** `edgedata = literal_eval(data)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 184. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/networkx/readwrite/tests/test_gml.py:441
- **Description:** eval()
- **Evidence (Line):** `data.append(literal_eval("{2.3j, 1 - 2.3j, ()}"))`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 185. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/networkx/tests/test_lazy_imports.py:83
- **Description:** exec()
- **Evidence (Line):** `exec(s, {}, locls)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 186. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/networkx/utils/decorators.py:912
- **Description:** exec()
- **Evidence (Line):** `exec(compiled, globl, locl)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 187. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/networkx/utils/tests/test_backends.py:28
- **Description:** pickle load/dumps
- **Evidence (Line):** `assert pickle.loads(pickled) is func.__wrapped__`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 188. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/networkx/utils/tests/test_backends.py:34
- **Description:** pickle load/dumps
- **Evidence (Line):** `assert pickle.loads(pickled) is func`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 189. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/networkx/utils/tests/test_backends.py:37
- **Description:** pickle load/dumps
- **Evidence (Line):** `assert pickle.loads(pickle.dumps(nx.inverse_line_graph)) is nx.inverse_line_graph`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 190. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/networkx/utils/tests/test_config.py:47
- **Description:** pickle load/dumps
- **Evidence (Line):** `cfg2 = pickle.loads(pickle.dumps(cfg))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 191. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/networkx/utils/tests/test_config.py:102
- **Description:** pickle load/dumps
- **Evidence (Line):** `cfg2 = pickle.loads(pickle.dumps(cfg))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 192. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/_internal.py:205
- **Description:** eval()
- **Evidence (Line):** `newitem = (dtype, ast.literal_eval(repeats))`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 193. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/arrayprint.py:1568
- **Description:** eval()
- **Evidence (Line):** `>>> assert eval(dtype_short_repr(dt)) == dt`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 194. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/records.py:702
- **Description:** pickle load/dumps
- **Evidence (Line):** `>>> pickle.loads(pickle.dumps(r))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 195. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test__exceptions.py:19
- **Description:** pickle load/dumps
- **Evidence (Line):** `res = pickle.loads(pickle.dumps(error))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 196. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test__exceptions.py:84
- **Description:** pickle load/dumps
- **Evidence (Line):** `exc2 = pickle.loads(pickle.dumps(exc))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 197. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_arrayprint.py:339
- **Description:** eval()
- **Evidence (Line):** `assert_equal(eval(repr(a), vars(np)), a)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 198. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_arrayprint.py:340
- **Description:** eval()
- **Evidence (Line):** `assert_equal(eval(repr(a[0]), {'np': np}), a[0])`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 199. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_custom_dtypes.py:386
- **Description:** pickle load/dumps
- **Evidence (Line):** `res = pickle.loads(s)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 200. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_datetime.py:866
- **Description:** pickle load/dumps
- **Evidence (Line):** `assert_equal(pickle.loads(pickle.dumps(dt, protocol=proto)), dt)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 201. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_datetime.py:868
- **Description:** pickle load/dumps
- **Evidence (Line):** `assert_equal(pickle.loads(pickle.dumps(dt, protocol=proto)), dt)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 202. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_datetime.py:870
- **Description:** pickle load/dumps
- **Evidence (Line):** `assert_equal(pickle.loads(pickle.dumps(scalar, protocol=proto)),`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 203. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_datetime.py:873
- **Description:** pickle load/dumps
- **Evidence (Line):** `assert_equal(pickle.loads(pickle.dumps(delta, protocol=proto)),`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 204. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_datetime.py:880
- **Description:** pickle load/dumps
- **Evidence (Line):** `assert_equal(pickle.loads(pkl), np.dtype('<M8[7D]'))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 205. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_datetime.py:884
- **Description:** pickle load/dumps
- **Evidence (Line):** `assert_equal(pickle.loads(pkl), np.dtype('<M8[W]'))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 206. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_datetime.py:888
- **Description:** pickle load/dumps
- **Evidence (Line):** `assert_equal(pickle.loads(pkl), np.dtype('>M8[us]'))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 207. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_datetime.py:895
- **Description:** pickle load/dumps
- **Evidence (Line):** `res = pickle.loads(pickle.dumps(dt, protocol=proto))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 208. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_deprecations.py:284
- **Description:** eval()
- **Evidence (Line):** `self.assert_deprecated(lambda: safe_eval("None"))`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 209. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_dtype.py:1070
- **Description:** eval()
- **Evidence (Line):** `assert_equal(np.dtype(eval(str(dt))), dt)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 210. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_dtype.py:1372
- **Description:** pickle load/dumps
- **Evidence (Line):** `pickled = pickle.loads(buf)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 211. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_dtype.py:1443
- **Description:** pickle load/dumps
- **Evidence (Line):** `roundtrip_DType = pickle.loads(pickle.dumps(DType, proto))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 212. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_dtype.py:1454
- **Description:** pickle load/dumps
- **Evidence (Line):** `roundtrip_dt = pickle.loads(pickle.dumps(dt, proto))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 213. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_multiarray.py:1665
- **Description:** eval()
- **Evidence (Line):** `assert_array_equal(eval(repr(xx), {"np": np, "array": np.array}), xx)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 214. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_multiarray.py:4093
- **Description:** eval()
- **Evidence (Line):** `MyType = eval("type('MyType', (base,), class_namespace)",`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 215. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_multiarray.py:190
- **Description:** pickle load/dumps
- **Evidence (Line):** `vals = pickle.loads(pickle.dumps(a, v))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 216. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_multiarray.py:1817
- **Description:** pickle load/dumps
- **Evidence (Line):** `scalar = pickle.loads(pickle.dumps(scalar))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 217. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_multiarray.py:1969
- **Description:** pickle load/dumps
- **Evidence (Line):** `assert_equal(zs, pickle.loads(zs.dumps()))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 218. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_multiarray.py:1976
- **Description:** pickle load/dumps
- **Evidence (Line):** `zs2 = pickle.loads(p)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 219. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_multiarray.py:1985
- **Description:** pickle load/dumps
- **Evidence (Line):** `pk_load = pickle.loads(pk_dmp)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 220. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_multiarray.py:1996
- **Description:** pickle load/dumps
- **Evidence (Line):** `array_from_buffer = pickle.loads(bytes_string, buffers=buffers)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 221. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_multiarray.py:4589
- **Description:** pickle load/dumps
- **Evidence (Line):** `depickled_arr_with_object = pickle.loads(`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 222. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_multiarray.py:4591
- **Description:** pickle load/dumps
- **Evidence (Line):** `depickled_arr_without_object = pickle.loads(`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 223. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_multiarray.py:4612
- **Description:** pickle load/dumps
- **Evidence (Line):** `depickled_f_contiguous_array = pickle.loads(bytes_string,`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 224. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_multiarray.py:4631
- **Description:** pickle load/dumps
- **Evidence (Line):** `depickled_transposed_contiguous_array = pickle.loads(bytes_string,`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 225. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_multiarray.py:4644
- **Description:** pickle load/dumps
- **Evidence (Line):** `assert_equal(x, pickle.loads(c_contiguous_dumped))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 226. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_multiarray.py:4646
- **Description:** pickle load/dumps
- **Evidence (Line):** `assert_equal(x, pickle.loads(f_contiguous_dumped))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 227. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_multiarray.py:4648
- **Description:** pickle load/dumps
- **Evidence (Line):** `assert_equal(x, pickle.loads(transposed_contiguous_dumped))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 228. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_multiarray.py:4650
- **Description:** pickle load/dumps
- **Evidence (Line):** `assert_equal(x, pickle.loads(no_contiguous_dumped))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 229. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_multiarray.py:4661
- **Description:** pickle load/dumps
- **Evidence (Line):** `depickled_non_contiguous_array = pickle.loads(`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 230. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_multiarray.py:4682
- **Description:** pickle load/dumps
- **Evidence (Line):** `a, pickle.loads(pickle.dumps(a, protocol=proto)),`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 231. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_multiarray.py:4691
- **Description:** pickle load/dumps
- **Evidence (Line):** `return pickle.loads(obj, encoding='latin1')`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 232. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_multiarray.py:4746
- **Description:** pickle load/dumps
- **Evidence (Line):** `new = pickle.loads(pickle.dumps(original_byte_reversed))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 233. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_multiarray.py:8639
- **Description:** pickle load/dumps
- **Evidence (Line):** `res = pickle.loads(pickle_obj)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 234. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_overrides.py:221
- **Description:** pickle load/dumps
- **Evidence (Line):** `roundtripped = pickle.loads(`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 235. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_records.py:170
- **Description:** eval()
- **Evidence (Line):** `recordarr_r = eval("np." + repr(recordarr), {'np': np})`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 236. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_records.py:171
- **Description:** eval()
- **Evidence (Line):** `recarr_r = eval("np." + repr(recarr), {'np': np})`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 237. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_records.py:173
- **Description:** eval()
- **Evidence (Line):** `recordview_r = eval("np." + repr(recordview), {'np': np, 'numpy': np})`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 238. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_records.py:414
- **Description:** pickle load/dumps
- **Evidence (Line):** `assert_equal(a, pickle.loads(pickle.dumps(a, protocol=proto)))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 239. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_records.py:415
- **Description:** pickle load/dumps
- **Evidence (Line):** `assert_equal(a[0], pickle.loads(pickle.dumps(a[0],`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 240. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_records.py:421
- **Description:** pickle load/dumps
- **Evidence (Line):** `assert_equal(a, pickle.loads(pickle.dumps(a, protocol=proto)))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 241. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_records.py:422
- **Description:** pickle load/dumps
- **Evidence (Line):** `assert_equal(a[0], pickle.loads(pickle.dumps(a[0],`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 242. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_records.py:429
- **Description:** pickle load/dumps
- **Evidence (Line):** `pa = pickle.loads(pickle.dumps(a[0], protocol=proto))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 243. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_records.py:453
- **Description:** pickle load/dumps
- **Evidence (Line):** `unpickled = pickle.loads(dump)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 244. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_regression.py:52
- **Description:** pickle load/dumps
- **Evidence (Line):** `b = pickle.load(f)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 245. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_regression.py:363
- **Description:** pickle load/dumps
- **Evidence (Line):** `dt_ = pickle.load(f)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 246. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_regression.py:488
- **Description:** pickle load/dumps
- **Evidence (Line):** `result = pickle.loads(data, encoding='bytes')`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 247. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_regression.py:832
- **Description:** pickle load/dumps
- **Evidence (Line):** `new = pickle.loads(pickle.dumps(el, protocol=proto))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 248. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_regression.py:1068
- **Description:** pickle load/dumps
- **Evidence (Line):** `y = pickle.loads(pickle.dumps(x, protocol=proto))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 249. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_regression.py:1083
- **Description:** pickle load/dumps
- **Evidence (Line):** `xp = pickle.load(f, encoding='latin1')`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 250. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_regression.py:1276
- **Description:** pickle load/dumps
- **Evidence (Line):** `assert_(pickle.loads(`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 251. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_regression.py:1278
- **Description:** pickle load/dumps
- **Evidence (Line):** `assert_(pickle.loads(`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 252. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_regression.py:1909
- **Description:** pickle load/dumps
- **Evidence (Line):** `data = pickle.loads(blob)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 253. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_regression.py:1921
- **Description:** pickle load/dumps
- **Evidence (Line):** `data = pickle.loads(pickle.dumps(data, protocol=proto))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 254. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_regression.py:1935
- **Description:** pickle load/dumps
- **Evidence (Line):** `result = pickle.loads(data, encoding='latin1')`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 255. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_regression.py:1963
- **Description:** pickle load/dumps
- **Evidence (Line):** `result = pickle.loads(data, encoding='latin1')`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 256. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_regression.py:1972
- **Description:** pickle load/dumps
- **Evidence (Line):** `result = pickle.loads(data, encoding='koi8-r')`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 257. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_regression.py:2218
- **Description:** pickle load/dumps
- **Evidence (Line):** `assert_equal(pickle.loads(`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 258. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_regression.py:2442
- **Description:** pickle load/dumps
- **Evidence (Line):** `assert_equal(pickle.loads(dumped), arr)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 259. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_regression.py:2574
- **Description:** pickle load/dumps
- **Evidence (Line):** `result = pickle.loads(test_data, encoding='bytes')`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 260. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_scalarmath.py:618
- **Description:** eval()
- **Evidence (Line):** `#            val2 = eval(val_repr)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 261. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_scalarmath.py:644
- **Description:** eval()
- **Evidence (Line):** `val2 = t(eval(val_repr))`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 262. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_simd.py:244
- **Description:** eval()
- **Evidence (Line):** `func = eval(intrin)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 263. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_simd.py:512
- **Description:** eval()
- **Evidence (Line):** `func = eval(intrin[:3])`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 264. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_simd.py:642
- **Description:** eval()
- **Evidence (Line):** `func = eval(intrin)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 265. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_simd.py:703
- **Description:** eval()
- **Evidence (Line):** `npyv_load_tillz, npyv_load_till = eval(intrin)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 266. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_simd.py:723
- **Description:** eval()
- **Evidence (Line):** `npyv_store_till = eval(intrin)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 267. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_simd.py:743
- **Description:** eval()
- **Evidence (Line):** `npyv_loadn = eval(intrin)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 268. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_simd.py:769
- **Description:** eval()
- **Evidence (Line):** `npyv_loadn_tillz, npyv_loadn_till = eval(intrin)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 269. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_simd.py:806
- **Description:** eval()
- **Evidence (Line):** `npyv_storen = eval(intrin)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 270. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_simd.py:845
- **Description:** eval()
- **Evidence (Line):** `npyv_storen_till = eval(intrin)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 271. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_simd.py:897
- **Description:** eval()
- **Evidence (Line):** `intrin = eval(intrin)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 272. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_simd.py:1104
- **Description:** eval()
- **Evidence (Line):** `func = eval(intrin)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 273. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_stringdtype.py:408
- **Description:** pickle load/dumps
- **Evidence (Line):** `res = pickle.load(f)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 274. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_ufunc.py:204
- **Description:** pickle load/dumps
- **Evidence (Line):** `assert_(pickle.loads(pickle.dumps(np.sin,`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 275. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_ufunc.py:209
- **Description:** pickle load/dumps
- **Evidence (Line):** `res = pickle.loads(pickle.dumps(_rational_tests.test_add,`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 276. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_ufunc.py:216
- **Description:** pickle load/dumps
- **Evidence (Line):** `assert_(pickle.loads(astring) is np.cos)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 277. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_ufunc.py:226
- **Description:** pickle load/dumps
- **Evidence (Line):** `obj = pickle.loads(pickle.dumps(_pickleable_module_global.ufunc))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 278. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_ufunc.py:501
- **Description:** pickle load/dumps
- **Evidence (Line):** `int64_2 = pickle.loads(pickle.dumps(int64))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 279. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_umath.py:512
- **Description:** eval()
- **Evidence (Line):** `a, b, divisors = eval(ex_val)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 280. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_umath.py:576
- **Description:** eval()
- **Evidence (Line):** `a = eval(ex_val)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 281. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_umath_accuracy.py:75
- **Description:** eval()
- **Evidence (Line):** `dtype=eval(datatype))`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 282. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/_core/tests/test_umath_accuracy.py:78
- **Description:** eval()
- **Evidence (Line):** `dtype=eval(datatype))`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 283. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/distutils/_shell_utils.py:73
- **Description:** subprocess with shell=True
- **Evidence (Line):** `The parsing behavior used by `subprocess.call("string", shell=True)` on Posix.`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 284. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/distutils/misc_util.py:157
- **Description:** eval()
- **Evidence (Line):** `caller_file = eval('__file__', frame.f_globals, frame.f_locals)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 285. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/distutils/misc_util.py:163
- **Description:** eval()
- **Evidence (Line):** `caller_name = eval('__name__', frame.f_globals, frame.f_locals)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 286. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/distutils/misc_util.py:847
- **Description:** eval()
- **Evidence (Line):** `caller_instance = eval('self', f.f_globals, f.f_locals)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 287. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/distutils/tests/test_ccompiler_opt.py:805
- **Description:** exec()
- **Evidence (Line):** `exec(new_test(arch, cc))`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 288. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/distutils/tests/test_shell_utils.py:48
- **Description:** subprocess with shell=True
- **Evidence (Line):** `return lambda cmd: subprocess.check_output(cmd, shell=True)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 289. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/f2py/auxfuncs.py:632
- **Description:** eval()
- **Evidence (Line):** `return eval(f"{l1}:{' and '.join(l2)}")`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 290. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/f2py/auxfuncs.py:640
- **Description:** eval()
- **Evidence (Line):** `return eval(f"{l1}:{' or '.join(l2)}")`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 291. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/f2py/auxfuncs.py:644
- **Description:** eval()
- **Evidence (Line):** `return eval('lambda v,f=f:not f(v)')`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 292. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/f2py/capi_maps.py:159
- **Description:** eval()
- **Evidence (Line):** `d = eval(f.read().lower(), {}, {})`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 293. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/f2py/capi_maps.py:296
- **Description:** eval()
- **Evidence (Line):** `ret['size'] = repr(eval(ret['size']))`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 294. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/f2py/capi_maps.py:449
- **Description:** eval()
- **Evidence (Line):** `v = eval(v, {}, {})`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 295. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/f2py/crackfortran.py:1328
- **Description:** eval()
- **Evidence (Line):** `v = eval(initexpr, {}, params)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 296. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/f2py/crackfortran.py:2268
- **Description:** eval()
- **Evidence (Line):** `def myeval(e, g=None, l=None):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 297. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/f2py/crackfortran.py:2270
- **Description:** eval()
- **Evidence (Line):** `r = eval(e, g, l)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 298. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/f2py/crackfortran.py:2301
- **Description:** eval()
- **Evidence (Line):** `c = int(myeval(e, {}, {}))`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 299. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/f2py/crackfortran.py:2322
- **Description:** eval()
- **Evidence (Line):** `b = myeval(ee, {}, {})`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 300. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/f2py/crackfortran.py:2327
- **Description:** eval()
- **Evidence (Line):** `a = myeval(ee, {}, {}) - b`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 301. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/f2py/crackfortran.py:2332
- **Description:** eval()
- **Evidence (Line):** `c = myeval(ee, {}, {})`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 302. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/f2py/crackfortran.py:2338
- **Description:** eval()
- **Evidence (Line):** `c2 = myeval(ee, {}, {})`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 303. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/f2py/crackfortran.py:2528
- **Description:** eval()
- **Evidence (Line):** `params[n] = param_eval(v, g_params, params, dimspec=dimspec)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 304. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/f2py/crackfortran.py:2558
- **Description:** eval()
- **Evidence (Line):** `value = eval(value, {}, params)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 305. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/f2py/crackfortran.py:2636
- **Description:** eval()
- **Evidence (Line):** `l = str(eval(l, {}, params))`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 306. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/f2py/crackfortran.py:2645
- **Description:** eval()
- **Evidence (Line):** `l = str(eval(l, {}, params))`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 307. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/f2py/crackfortran.py:2913
- **Description:** eval()
- **Evidence (Line):** `kindselect['kind'] = eval(`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 308. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/f2py/crackfortran.py:2963
- **Description:** eval()
- **Evidence (Line):** `def param_eval(v, g_params, params, dimspec=None):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 309. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/f2py/crackfortran.py:2984
- **Description:** eval()
- **Evidence (Line):** `p = eval(v, g_params, params)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 310. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/f2py/crackfortran.py:3015
- **Description:** eval()
- **Evidence (Line):** `item = eval(item, g_params, params)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 311. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/f2py/crackfortran.py:3467
- **Description:** eval()
- **Evidence (Line):** `v = eval(v)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 312. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/f2py/tests/test_crackfortran.py:372
- **Description:** eval()
- **Evidence (Line):** `ret = crackfortran.param_eval(v, g_params, params, dimspec=dimspec)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 313. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/f2py/tests/test_crackfortran.py:382
- **Description:** eval()
- **Evidence (Line):** `ret = crackfortran.param_eval(v, g_params, params, dimspec=dimspec)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 314. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/f2py/tests/test_crackfortran.py:401
- **Description:** eval()
- **Evidence (Line):** `ret = crackfortran.param_eval(v, g_params, params, dimspec=None)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 315. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/f2py/tests/test_f2py2e.py:866
- **Description:** exec()
- **Evidence (Line):** `def test_npd_f77exec():`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 316. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/f2py/tests/test_f2py2e.py:874
- **Description:** exec()
- **Evidence (Line):** `def test_npd_f90exec():`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 317. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/lib/_format_impl.py:536
- **Description:** eval()
- **Evidence (Line):** `See :py:func:`ast.literal_eval()` for details.`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 318. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/lib/_format_impl.py:563
- **Description:** eval()
- **Evidence (Line):** `See :py:func:`ast.literal_eval()` for details.`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 319. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/lib/_format_impl.py:658
- **Description:** eval()
- **Evidence (Line):** `d = ast.literal_eval(header)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 320. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/lib/_format_impl.py:663
- **Description:** eval()
- **Evidence (Line):** `d = ast.literal_eval(header)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 321. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/lib/_format_impl.py:799
- **Description:** eval()
- **Evidence (Line):** `See :py:func:`ast.literal_eval()` for details.`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 322. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/lib/_format_impl.py:927
- **Description:** eval()
- **Evidence (Line):** `See :py:func:`ast.literal_eval()` for details.`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 323. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/lib/_format_impl.py:838
- **Description:** pickle load/dumps
- **Evidence (Line):** `array = pickle.load(fp, **pickle_kwargs)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 324. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/lib/_npyio_impl.py:151
- **Description:** eval()
- **Evidence (Line):** `See :py:func:`ast.literal_eval()` for details.`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 325. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/lib/_npyio_impl.py:356
- **Description:** eval()
- **Evidence (Line):** `See :py:func:`ast.literal_eval()` for details.`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 326. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/lib/_npyio_impl.py:492
- **Description:** pickle load/dumps
- **Evidence (Line):** `"`allow_pickle=` keyword argument or `pickle.load()`.")`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 327. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/lib/_npyio_impl.py:494
- **Description:** pickle load/dumps
- **Evidence (Line):** `return pickle.load(fid, **pickle_kwargs)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 328. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/lib/_utils_impl.py:583
- **Description:** eval()
- **Evidence (Line):** `def safe_eval(source):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 329. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/lib/_utils_impl.py:617
- **Description:** eval()
- **Evidence (Line):** `>>> np.safe_eval('1')`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 330. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/lib/_utils_impl.py:619
- **Description:** eval()
- **Evidence (Line):** `>>> np.safe_eval('[1, 2, 3]')`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 331. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/lib/_utils_impl.py:621
- **Description:** eval()
- **Evidence (Line):** `>>> np.safe_eval('{"foo": ("bar", 10.0)}')`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 332. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/lib/_utils_impl.py:624
- **Description:** eval()
- **Evidence (Line):** `>>> np.safe_eval('import os')`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 333. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/lib/_utils_impl.py:629
- **Description:** eval()
- **Evidence (Line):** `>>> np.safe_eval('open("/home/user/.ssh/id_dsa").read()')`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 334. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/lib/_utils_impl.py:647
- **Description:** eval()
- **Evidence (Line):** `return ast.literal_eval(source)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 335. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/ma/tests/test_core.py:730
- **Description:** pickle load/dumps
- **Evidence (Line):** `a_pickled = pickle.loads(pickle.dumps(a, protocol=proto))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 336. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/ma/tests/test_core.py:745
- **Description:** pickle load/dumps
- **Evidence (Line):** `a_pickled = pickle.loads(pickle.dumps(a, protocol=proto))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 337. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/ma/tests/test_core.py:754
- **Description:** pickle load/dumps
- **Evidence (Line):** `mc_pickled = pickle.loads(pickle.dumps(mc, protocol=proto))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 338. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/ma/tests/test_core.py:764
- **Description:** pickle load/dumps
- **Evidence (Line):** `a_pickled = pickle.loads(pickle.dumps(a, protocol=proto))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 339. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/ma/tests/test_core.py:773
- **Description:** pickle load/dumps
- **Evidence (Line):** `test = pickle.loads(pickle.dumps(b, protocol=proto))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 340. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/ma/tests/test_core.py:5571
- **Description:** pickle load/dumps
- **Evidence (Line):** `res = pickle.load(f)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 341. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/ma/tests/test_mrecords.py:291
- **Description:** pickle load/dumps
- **Evidence (Line):** `mrec_ = pickle.loads(_)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 342. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/ma/tests/test_old_ma.py:617
- **Description:** pickle load/dumps
- **Evidence (Line):** `y = pickle.loads(s)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 343. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/matrixlib/tests/test_masked_matrix.py:89
- **Description:** pickle load/dumps
- **Evidence (Line):** `a_pickled = pickle.loads(pickle.dumps(a, protocol=proto))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 344. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/polynomial/hermite_e.py:244
- **Description:** eval()
- **Evidence (Line):** `>>> hermeval(0,hermeline(3, 2))`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 345. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/polynomial/hermite_e.py:246
- **Description:** eval()
- **Evidence (Line):** `>>> hermeval(1,hermeline(3, 2))`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 346. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/polynomial/hermite_e.py:302
- **Description:** eval()
- **Evidence (Line):** `>>> hermeval((-1, 0, 1), coef)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 347. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/polynomial/hermite_e.py:305
- **Description:** eval()
- **Evidence (Line):** `>>> hermeval((-1j, 1j), coef)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 348. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/polynomial/hermite_e.py:789
- **Description:** eval()
- **Evidence (Line):** `tmp[0] += k[i] - hermeval(lbnd, tmp)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 349. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/polynomial/hermite_e.py:795
- **Description:** eval()
- **Evidence (Line):** `def hermeval(x, c, tensor=True):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 350. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/polynomial/hermite_e.py:855
- **Description:** eval()
- **Evidence (Line):** `>>> hermeval(1, coef)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 351. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/polynomial/hermite_e.py:857
- **Description:** eval()
- **Evidence (Line):** `>>> hermeval([[1,2],[3,4]], coef)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 352. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/polynomial/hermite_e.py:1086
- **Description:** eval()
- **Evidence (Line):** ```hermeval(x, c)`` are the same up to roundoff. This equivalence is`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 353. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/polynomial/hermite_e.py:1355
- **Description:** eval()
- **Evidence (Line):** `>>> y = hermeval(x, [1, 2, 3]) + err`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 354. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/polynomial/tests/test_hermite_e.py:79
- **Description:** eval()
- **Evidence (Line):** `val1 = herme.hermeval(self.x, pol1)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 355. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/polynomial/tests/test_hermite_e.py:83
- **Description:** eval()
- **Evidence (Line):** `val2 = herme.hermeval(self.x, pol2)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 356. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/polynomial/tests/test_hermite_e.py:85
- **Description:** eval()
- **Evidence (Line):** `val3 = herme.hermeval(self.x, pol3)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 357. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/polynomial/tests/test_hermite_e.py:120
- **Description:** eval()
- **Evidence (Line):** `def test_hermeval(self):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 358. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/polynomial/tests/test_hermite_e.py:122
- **Description:** eval()
- **Evidence (Line):** `assert_equal(herme.hermeval([], [1]).size, 0)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 359. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/polynomial/tests/test_hermite_e.py:130
- **Description:** eval()
- **Evidence (Line):** `res = herme.hermeval(x, [0] * i + [1])`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 360. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/polynomial/tests/test_hermite_e.py:137
- **Description:** eval()
- **Evidence (Line):** `assert_equal(herme.hermeval(x, [1]).shape, dims)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 361. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/polynomial/tests/test_hermite_e.py:138
- **Description:** eval()
- **Evidence (Line):** `assert_equal(herme.hermeval(x, [1, 0]).shape, dims)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 362. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/polynomial/tests/test_hermite_e.py:139
- **Description:** eval()
- **Evidence (Line):** `assert_equal(herme.hermeval(x, [1, 0, 0]).shape, dims)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 363. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/polynomial/tests/test_hermite_e.py:237
- **Description:** eval()
- **Evidence (Line):** `assert_almost_equal(herme.hermeval(-1, hermeint), i)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 364. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/polynomial/tests/test_hermite_e.py:358
- **Description:** eval()
- **Evidence (Line):** `assert_almost_equal(v[..., i], herme.hermeval(x, coef))`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 365. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/polynomial/tests/test_hermite_e.py:366
- **Description:** eval()
- **Evidence (Line):** `assert_almost_equal(v[..., i], herme.hermeval(x, coef))`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 366. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/polynomial/tests/test_hermite_e.py:423
- **Description:** eval()
- **Evidence (Line):** `assert_almost_equal(herme.hermeval(x, coef3), y)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 367. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/polynomial/tests/test_hermite_e.py:426
- **Description:** eval()
- **Evidence (Line):** `assert_almost_equal(herme.hermeval(x, coef3), y)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 368. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/polynomial/tests/test_hermite_e.py:430
- **Description:** eval()
- **Evidence (Line):** `assert_almost_equal(herme.hermeval(x, coef4), y)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 369. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/polynomial/tests/test_hermite_e.py:433
- **Description:** eval()
- **Evidence (Line):** `assert_almost_equal(herme.hermeval(x, coef4), y)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 370. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/polynomial/tests/test_hermite_e.py:437
- **Description:** eval()
- **Evidence (Line):** `assert_almost_equal(herme.hermeval(x, coef4), y)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 371. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/polynomial/tests/test_hermite_e.py:466
- **Description:** eval()
- **Evidence (Line):** `assert_almost_equal(herme.hermeval(x, coef1), y)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 372. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/polynomial/tests/test_hermite_e.py:468
- **Description:** eval()
- **Evidence (Line):** `assert_almost_equal(herme.hermeval(x, coef2), y)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 373. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/polynomial/tests/test_hermite_e.py:514
- **Description:** eval()
- **Evidence (Line):** `res = herme.hermeval(roots, pol)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 374. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/polynomial/tests/test_polynomial.py:62
- **Description:** pickle load/dumps
- **Evidence (Line):** `y = pickle.loads(pickle.dumps(x))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 375. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/random/tests/test_direct.py:303
- **Description:** pickle load/dumps
- **Evidence (Line):** `reloaded = pickle.loads(bitgen_pkl)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 376. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/random/tests/test_direct.py:311
- **Description:** pickle load/dumps
- **Evidence (Line):** `aa = pickle.loads(pickle.dumps(ss))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 377. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/random/tests/test_direct.py:321
- **Description:** pickle load/dumps
- **Evidence (Line):** `bg_plk = pickle.loads(pickle.dumps(bit_generator))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 378. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/random/tests/test_direct.py:327
- **Description:** pickle load/dumps
- **Evidence (Line):** `bg_plk = pickle.loads(pickle.dumps(bit_generator))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 379. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/random/tests/test_direct.py:555
- **Description:** pickle load/dumps
- **Evidence (Line):** `sfc = pickle.load(gz)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 380. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/random/tests/test_generator_mt19937.py:2797
- **Description:** pickle load/dumps
- **Evidence (Line):** `rg_plk = pickle.loads(pickle.dumps(rg))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 381. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/random/tests/test_generator_mt19937.py:2803
- **Description:** pickle load/dumps
- **Evidence (Line):** `rg_plk = pickle.loads(pickle.dumps(rg))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 382. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/random/tests/test_generator_mt19937.py:2819
- **Description:** pickle load/dumps
- **Evidence (Line):** `rg = pickle.load(gz)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 383. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/random/tests/test_randomstate.py:272
- **Description:** pickle load/dumps
- **Evidence (Line):** `rs_unpick = pickle.loads(pickle.dumps(random_state))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 384. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/random/tests/test_smoke.py:488
- **Description:** pickle load/dumps
- **Evidence (Line):** `unpick = pickle.loads(pick)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 385. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/random/tests/test_smoke.py:494
- **Description:** pickle load/dumps
- **Evidence (Line):** `unpick = pickle.loads(pick)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 386. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/testing/_private/utils.py:1346
- **Description:** exec()
- **Evidence (Line):** `exec(astr, dict)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 387. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/testing/_private/utils.py:1632
- **Description:** exec()
- **Evidence (Line):** `exec(code, globs, locs)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 388. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/tests/test_public_api.py:405
- **Description:** eval()
- **Evidence (Line):** `eval(module_name)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 389. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/tests/test_reloading.py:40
- **Description:** pickle load/dumps
- **Evidence (Line):** `assert_(pickle.loads(pickle.dumps(np._NoValue,`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 390. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/packaging/_parser.py:397
- **Description:** eval()
- **Evidence (Line):** `value = ast.literal_eval(python_str)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 391. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/packaging/_structures.py:10
- **Description:** pickle load/dumps
- **Evidence (Line):** `stand-in classes so that ``pickle.loads()`` can resolve those references.`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 392. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/core/computation/eval.py:173
- **Description:** eval()
- **Evidence (Line):** `def eval(`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 393. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/core/computation/eval.py:297
- **Description:** eval()
- **Evidence (Line):** `>>> pd.eval("double_age = df.age * 2", target=df)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 394. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/core/computation/expr.py:482
- **Description:** eval()
- **Evidence (Line):** `def _maybe_eval(self, binop, eval_in_python):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 395. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/core/computation/expr.py:517
- **Description:** eval()
- **Evidence (Line):** `return self._maybe_eval(res, self.binary_ops)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 396. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/core/computation/expr.py:521
- **Description:** eval()
- **Evidence (Line):** `return self._maybe_eval(res, eval_in_python)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 397. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/core/computation/expr.py:531
- **Description:** eval()
- **Evidence (Line):** `return self._maybe_eval(res, eval_in_python + maybe_eval_in_python)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 398. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/core/computation/expr.py:578
- **Description:** eval()
- **Evidence (Line):** `result = pd_eval(`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 399. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/core/computation/expr.py:586
- **Description:** eval()
- **Evidence (Line):** `lhs = pd_eval(`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 400. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/core/computation/ops.py:431
- **Description:** eval()
- **Evidence (Line):** `res = eval(self, local_dict=env, engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 401. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/core/computation/scope.py:199
- **Description:** eval()
- **Evidence (Line):** ```DataFrame.eval()`` and ``DataFrame.query()``.`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 402. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/core/frame.py:4834
- **Description:** eval()
- **Evidence (Line):** `res = self.eval(expr, **kwargs)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 403. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/core/frame.py:4850
- **Description:** eval()
- **Evidence (Line):** `def eval(self, expr: str, *, inplace: Literal[False] = ..., **kwargs) -> Any:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 404. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/core/frame.py:4854
- **Description:** eval()
- **Evidence (Line):** `def eval(self, expr: str, *, inplace: Literal[True], **kwargs) -> None:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 405. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/core/frame.py:4857
- **Description:** eval()
- **Evidence (Line):** `def eval(self, expr: str, *, inplace: bool = False, **kwargs) -> Any | None:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 406. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/core/frame.py:4908
- **Description:** eval()
- **Evidence (Line):** `>>> df.eval('A + B')`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 407. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/core/frame.py:4919
- **Description:** eval()
- **Evidence (Line):** `>>> df.eval('C = A + B')`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 408. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/core/frame.py:4936
- **Description:** eval()
- **Evidence (Line):** `>>> df.eval(`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 409. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/core/frame.py:4960
- **Description:** eval()
- **Evidence (Line):** `return _eval(expr, inplace=inplace, **kwargs)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 410. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/errors/__init__.py:567
- **Description:** eval()
- **Evidence (Line):** `>>> pd.eval("sin + a", engine='numexpr') # doctest: +SKIP`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 411. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/errors/__init__.py:585
- **Description:** eval()
- **Evidence (Line):** `>>> pd.eval('x + 1') # doctest: +SKIP`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 412. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/io/pickle.py:202
- **Description:** pickle load/dumps
- **Evidence (Line):** `return pickle.load(handles.handle)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 413. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/arrays/string_/test_string_arrow.py:270
- **Description:** pickle load/dumps
- **Evidence (Line):** `result = pickle.loads(full_pickled)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 414. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/arrays/string_/test_string_arrow.py:273
- **Description:** pickle load/dumps
- **Evidence (Line):** `result_sliced = pickle.loads(sliced_pickled)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 415. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_compat.py:31
- **Description:** eval()
- **Evidence (Line):** `res = pd.eval("a + b", engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 416. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:150
- **Description:** eval()
- **Evidence (Line):** `pd.eval(ex, engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 417. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:158
- **Description:** eval()
- **Evidence (Line):** `result = pd.eval(ex, engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 418. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:171
- **Description:** eval()
- **Evidence (Line):** `pd.eval(ex, engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 419. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:185
- **Description:** eval()
- **Evidence (Line):** `pd.eval(`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 420. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:193
- **Description:** eval()
- **Evidence (Line):** `result = pd.eval(ex, engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 421. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:202
- **Description:** eval()
- **Evidence (Line):** `pd.eval(ex, engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 422. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:230
- **Description:** eval()
- **Evidence (Line):** `pd.eval(`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 423. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:245
- **Description:** eval()
- **Evidence (Line):** `result = pd.eval(ex, engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 424. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:256
- **Description:** eval()
- **Evidence (Line):** `pd.eval(ex1, engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 425. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:269
- **Description:** eval()
- **Evidence (Line):** `result = pd.eval(ex, engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 426. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:278
- **Description:** eval()
- **Evidence (Line):** `result = pd.eval(ex, engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 427. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:283
- **Description:** eval()
- **Evidence (Line):** `result = pd.eval(ex, engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 428. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:303
- **Description:** eval()
- **Evidence (Line):** `expected = eval(f"nlhs {arith1} ghs")`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 429. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:310
- **Description:** eval()
- **Evidence (Line):** `result = pd.eval(ex, engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 430. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:332
- **Description:** eval()
- **Evidence (Line):** `res = pd.eval(ex, engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 431. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:341
- **Description:** eval()
- **Evidence (Line):** `pd.eval(`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 432. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:353
- **Description:** eval()
- **Evidence (Line):** `result = pd.eval(ex, engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 433. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:368
- **Description:** eval()
- **Evidence (Line):** `result = pd.eval(ex, engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 434. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:381
- **Description:** eval()
- **Evidence (Line):** `result = pd.eval("~elb", engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 435. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:394
- **Description:** eval()
- **Evidence (Line):** `pd.eval(expr, engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 436. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:398
- **Description:** eval()
- **Evidence (Line):** `pd.eval(expr, engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 437. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:405
- **Description:** eval()
- **Evidence (Line):** `pd.eval(expr, engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 438. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:408
- **Description:** eval()
- **Evidence (Line):** `result = pd.eval(expr, engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 439. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:414
- **Description:** eval()
- **Evidence (Line):** `result = pd.eval(expr, engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 440. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:423
- **Description:** eval()
- **Evidence (Line):** `pd.eval(expr, engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 441. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:427
- **Description:** eval()
- **Evidence (Line):** `pd.eval(expr, engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 442. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:439
- **Description:** eval()
- **Evidence (Line):** `result = pd.eval(expr, engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 443. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:443
- **Description:** eval()
- **Evidence (Line):** `pd.eval(expr, engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 444. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:450
- **Description:** eval()
- **Evidence (Line):** `pd.eval(expr, engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 445. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:453
- **Description:** eval()
- **Evidence (Line):** `result = pd.eval(expr, engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 446. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:459
- **Description:** eval()
- **Evidence (Line):** `result = pd.eval(expr, engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 447. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:470
- **Description:** eval()
- **Evidence (Line):** `pd.eval(expr, engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 448. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:474
- **Description:** eval()
- **Evidence (Line):** `pd.eval(expr, engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 449. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:482
- **Description:** eval()
- **Evidence (Line):** `result = pd.eval(expr, engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 450. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:488
- **Description:** eval()
- **Evidence (Line):** `result = pd.eval(expr, engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 451. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:496
- **Description:** eval()
- **Evidence (Line):** `pd.eval(expr, engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 452. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:499
- **Description:** eval()
- **Evidence (Line):** `result = pd.eval(expr, engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 453. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:508
- **Description:** eval()
- **Evidence (Line):** `result = pd.eval(expr, engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 454. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:514
- **Description:** eval()
- **Evidence (Line):** `result = pd.eval(expr, engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 455. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:522
- **Description:** eval()
- **Evidence (Line):** `pd.eval(expr, engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 456. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:525
- **Description:** eval()
- **Evidence (Line):** `result = pd.eval(expr, engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 457. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:543
- **Description:** eval()
- **Evidence (Line):** `result = pd.eval(expr, engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 458. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:561
- **Description:** eval()
- **Evidence (Line):** `result = pd.eval(expr, engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 459. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:570
- **Description:** eval()
- **Evidence (Line):** `pd.eval("~1.0", engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 460. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:572
- **Description:** eval()
- **Evidence (Line):** `assert pd.eval("-1.0", parser=parser, engine=engine) == -1.0`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 461. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:573
- **Description:** eval()
- **Evidence (Line):** `assert pd.eval("+1.0", parser=parser, engine=engine) == +1.0`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 462. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:574
- **Description:** eval()
- **Evidence (Line):** `assert pd.eval("~1", parser=parser, engine=engine) == ~1`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 463. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:575
- **Description:** eval()
- **Evidence (Line):** `assert pd.eval("-1", parser=parser, engine=engine) == -1`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 464. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:576
- **Description:** eval()
- **Evidence (Line):** `assert pd.eval("+1", parser=parser, engine=engine) == +1`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 465. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:580
- **Description:** eval()
- **Evidence (Line):** `assert pd.eval("~True", parser=parser, engine=engine) == ~True`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 466. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:584
- **Description:** eval()
- **Evidence (Line):** `assert pd.eval("~False", parser=parser, engine=engine) == ~False`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 467. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:585
- **Description:** eval()
- **Evidence (Line):** `assert pd.eval("-True", parser=parser, engine=engine) == -True`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 468. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:586
- **Description:** eval()
- **Evidence (Line):** `assert pd.eval("-False", parser=parser, engine=engine) == -False`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 469. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:587
- **Description:** eval()
- **Evidence (Line):** `assert pd.eval("+True", parser=parser, engine=engine) == +True`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 470. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:588
- **Description:** eval()
- **Evidence (Line):** `assert pd.eval("+False", parser=parser, engine=engine) == +False`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 471. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:595
- **Description:** eval()
- **Evidence (Line):** `pd.eval("[-True, True, +True, -False, False, +False, -37, 37, ~37, +37]"),`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 472. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:619
- **Description:** eval()
- **Evidence (Line):** `res = df.eval(expr)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 473. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:626
- **Description:** eval()
- **Evidence (Line):** `result = df.eval("x.fillna(-1)")`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 474. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:632
- **Description:** eval()
- **Evidence (Line):** `result = df.eval("x.shift(1, fill_value=-1)")`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 475. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:654
- **Description:** eval()
- **Evidence (Line):** `pd.eval(ex, engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 476. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:659
- **Description:** eval()
- **Evidence (Line):** `result = pd.eval("x", engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 477. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:664
- **Description:** eval()
- **Evidence (Line):** `result = pd.eval("x", engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 478. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:669
- **Description:** eval()
- **Evidence (Line):** `result = pd.eval("x", engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 479. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:675
- **Description:** eval()
- **Evidence (Line):** `result = pd.eval("x", engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 480. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:680
- **Description:** eval()
- **Evidence (Line):** `result = pd.eval("x", engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 481. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:685
- **Description:** eval()
- **Evidence (Line):** `result = pd.eval("x", engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 482. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:693
- **Description:** eval()
- **Evidence (Line):** `result = pd.eval(exp, engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 483. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:699
- **Description:** eval()
- **Evidence (Line):** `result = pd.eval(exp, engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 484. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:736
- **Description:** eval()
- **Evidence (Line):** `assert pd.eval("not True") == -2`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 485. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:737
- **Description:** eval()
- **Evidence (Line):** `assert pd.eval("not False") == -1`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 486. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:738
- **Description:** eval()
- **Evidence (Line):** `assert pd.eval("True and not True") == 0`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 487. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:743
- **Description:** eval()
- **Evidence (Line):** `assert pd.eval(f"{event.str.match('hello').a}")`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 488. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:744
- **Description:** eval()
- **Evidence (Line):** `assert pd.eval(f"{event.str.match('hello').a and event.str.match('hello').a}")`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 489. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:764
- **Description:** eval()
- **Evidence (Line):** `res = pd.eval(s, engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 490. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:774
- **Description:** eval()
- **Evidence (Line):** `tm.assert_frame_equal(res, eval(s), check_exact=False)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 491. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:796
- **Description:** eval()
- **Evidence (Line):** `res = pd.eval(s, engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 492. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:819
- **Description:** eval()
- **Evidence (Line):** `res = pd.eval("df + df2", engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 493. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:821
- **Description:** eval()
- **Evidence (Line):** `res = pd.eval("df + df2", engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 494. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:834
- **Description:** eval()
- **Evidence (Line):** `res = pd.eval("df < 2", engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 495. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:842
- **Description:** eval()
- **Evidence (Line):** `res = pd.eval("df < df3", engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 496. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:870
- **Description:** eval()
- **Evidence (Line):** `res = pd.eval("df + df2 + df3", engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 497. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:872
- **Description:** eval()
- **Evidence (Line):** `res = pd.eval("df + df2 + df3", engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 498. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:892
- **Description:** eval()
- **Evidence (Line):** `res = pd.eval("df + s", engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 499. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:894
- **Description:** eval()
- **Evidence (Line):** `res = pd.eval("df + s", engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 500. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:933
- **Description:** eval()
- **Evidence (Line):** `res = pd.eval("s + df", engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 501. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:935
- **Description:** eval()
- **Evidence (Line):** `res = pd.eval("s + df", engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 502. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:963
- **Description:** eval()
- **Evidence (Line):** `a = pd.eval(lhs, engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 503. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:965
- **Description:** eval()
- **Evidence (Line):** `b = pd.eval(rhs, engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 504. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:967
- **Description:** eval()
- **Evidence (Line):** `a = pd.eval(lhs, engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 505. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:968
- **Description:** eval()
- **Evidence (Line):** `b = pd.eval(rhs, engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 506. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1016
- **Description:** eval()
- **Evidence (Line):** `res = pd.eval("df2 + ser + df", engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 507. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1018
- **Description:** eval()
- **Evidence (Line):** `res = pd.eval("df2 + ser + df", engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 508. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1031
- **Description:** eval()
- **Evidence (Line):** `pd.eval("df + s", engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 509. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1035
- **Description:** eval()
- **Evidence (Line):** `pd.eval("df + s", engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 510. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1040
- **Description:** eval()
- **Evidence (Line):** `pd.eval("df + s", engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 511. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1053
- **Description:** eval()
- **Evidence (Line):** `pd.eval("df + s", engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 512. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1072
- **Description:** eval()
- **Evidence (Line):** `def eval(self, *args, **kwargs):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 513. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1074
- **Description:** eval()
- **Evidence (Line):** `return pd.eval(*args, **kwargs)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 514. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1097
- **Description:** eval()
- **Evidence (Line):** `pd.eval(ex, engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 515. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1100
- **Description:** eval()
- **Evidence (Line):** `x = self.eval(ex, engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 516. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1104
- **Description:** eval()
- **Evidence (Line):** `y = self.eval(ex2, local_dict={"x": x}, engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 517. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1108
- **Description:** eval()
- **Evidence (Line):** `y = self.eval(ex3, local_dict={"x": x}, engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 518. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1120
- **Description:** eval()
- **Evidence (Line):** `self.eval(ex)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 519. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1123
- **Description:** eval()
- **Evidence (Line):** `res = self.eval(ex)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 520. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1124
- **Description:** eval()
- **Evidence (Line):** `exp = eval(ex)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 521. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1136
- **Description:** eval()
- **Evidence (Line):** `self.eval(ex)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 522. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1139
- **Description:** eval()
- **Evidence (Line):** `res = self.eval(ex)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 523. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1140
- **Description:** eval()
- **Evidence (Line):** `exp = eval(ex)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 524. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1148
- **Description:** eval()
- **Evidence (Line):** `self.eval("x + y", local_dict={"x": x, "y": y})`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 525. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1151
- **Description:** eval()
- **Evidence (Line):** `x = self.eval("1")`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 526. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1156
- **Description:** eval()
- **Evidence (Line):** `df2 = self.eval("df", local_dict={"df": df})`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 527. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1162
- **Description:** eval()
- **Evidence (Line):** `self.eval("df[x > 2] > 2")`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 528. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1166
- **Description:** eval()
- **Evidence (Line):** `result = self.eval("(df + 1)[df > 2]", local_dict={"df": df})`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 529. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1183
- **Description:** eval()
- **Evidence (Line):** `tm.assert_series_equal(expec, self.eval(e, local_dict={"df": df}))`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 530. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1193
- **Description:** eval()
- **Evidence (Line):** `self.eval(expr1, local_dict={"df": df, "df2": df2})`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 531. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1201
- **Description:** eval()
- **Evidence (Line):** `df.eval("d c = a + b")`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 532. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1210
- **Description:** eval()
- **Evidence (Line):** `df.eval("d,c = a + b")`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 533. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1218
- **Description:** eval()
- **Evidence (Line):** `df.eval('Timestamp("20131001") = a + b')`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 534. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1227
- **Description:** eval()
- **Evidence (Line):** `df.eval("a = a + b", inplace=True)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 535. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1237
- **Description:** eval()
- **Evidence (Line):** `df.eval("c = a + b", inplace=True)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 536. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1246
- **Description:** eval()
- **Evidence (Line):** `df.eval("a = 1 + b", inplace=True)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 537. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1259
- **Description:** eval()
- **Evidence (Line):** `df.eval("a = a + b", inplace=True)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 538. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1269
- **Description:** eval()
- **Evidence (Line):** `df.eval("c = a + b", inplace=True)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 539. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1272
- **Description:** eval()
- **Evidence (Line):** `df.eval("c = a = b")`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 540. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1279
- **Description:** eval()
- **Evidence (Line):** `self.eval("c = df.a + df.b", local_dict={"df": df}, target=df, inplace=True)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 541. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1287
- **Description:** eval()
- **Evidence (Line):** `result = df.eval("a in [11, -32]")`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 542. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1300
- **Description:** eval()
- **Evidence (Line):** `actual = df.eval("c = a + b", inplace=False)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 543. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1314
- **Description:** eval()
- **Evidence (Line):** `answer = df.eval(`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 544. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1325
- **Description:** eval()
- **Evidence (Line):** `answer = df.eval(`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 545. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1337
- **Description:** eval()
- **Evidence (Line):** `df.eval(`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 546. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1351
- **Description:** eval()
- **Evidence (Line):** `df = df.eval(`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 547. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1361
- **Description:** eval()
- **Evidence (Line):** `df = df.eval(`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 548. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1377
- **Description:** eval()
- **Evidence (Line):** `answer = df.eval(`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 549. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1397
- **Description:** eval()
- **Evidence (Line):** `answer = df.eval(`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 550. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1417
- **Description:** eval()
- **Evidence (Line):** `answer = df.eval(`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 551. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1447
- **Description:** eval()
- **Evidence (Line):** `self.eval("a = 1 + 2", target=df, inplace=True)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 552. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1456
- **Description:** eval()
- **Evidence (Line):** `self.eval(expression, target=invalid_target, inplace=True)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 553. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1460
- **Description:** eval()
- **Evidence (Line):** `self.eval(expression, target=invalid_target, inplace=False)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 554. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1468
- **Description:** eval()
- **Evidence (Line):** `self.eval(expression, target=invalid_target, inplace=False)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 555. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1474
- **Description:** eval()
- **Evidence (Line):** `assert self.eval(expression, target=target, inplace=False) == 3`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 556. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1478
- **Description:** eval()
- **Evidence (Line):** `self.eval(expression, target=target, inplace=True)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 557. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1486
- **Description:** eval()
- **Evidence (Line):** `r = self.eval("df < 2", local_dict={"df": df})`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 558. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1497
- **Description:** eval()
- **Evidence (Line):** `r = self.eval("df[df < 2 + 3]", local_dict={"df": df})`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 559. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1506
- **Description:** eval()
- **Evidence (Line):** `r = self.eval("df[df[df < 2] < 2] + df * 2", local_dict={"df": df})`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 560. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1513
- **Description:** eval()
- **Evidence (Line):** `res = self.eval(`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 561. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1524
- **Description:** eval()
- **Evidence (Line):** `res = pd.eval("1 in [1, 2]", engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 562. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1527
- **Description:** eval()
- **Evidence (Line):** `res = pd.eval("2 in (1, 2)", engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 563. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1530
- **Description:** eval()
- **Evidence (Line):** `res = pd.eval("3 in (1, 2)", engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 564. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1533
- **Description:** eval()
- **Evidence (Line):** `res = pd.eval("3 not in (1, 2)", engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 565. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1536
- **Description:** eval()
- **Evidence (Line):** `res = pd.eval("[3] not in (1, 2)", engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 566. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1539
- **Description:** eval()
- **Evidence (Line):** `res = pd.eval("[3] in ([3], 2)", engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 567. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1542
- **Description:** eval()
- **Evidence (Line):** `res = pd.eval("[[3]] in [[[3]], 2]", engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 568. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1545
- **Description:** eval()
- **Evidence (Line):** `res = pd.eval("(3,) in [(3,), 2]", engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 569. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1548
- **Description:** eval()
- **Evidence (Line):** `res = pd.eval("(3,) not in [(3,), 2]", engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 570. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1551
- **Description:** eval()
- **Evidence (Line):** `res = pd.eval("[(3,)] in [[(3,)], 2]", engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 571. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1556
- **Description:** eval()
- **Evidence (Line):** `pd.eval("1 in [1, 2]", engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 572. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1558
- **Description:** eval()
- **Evidence (Line):** `pd.eval("2 in (1, 2)", engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 573. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1560
- **Description:** eval()
- **Evidence (Line):** `pd.eval("3 in (1, 2)", engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 574. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1562
- **Description:** eval()
- **Evidence (Line):** `pd.eval("[(3,)] in (1, 2, [(3,)])", engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 575. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1565
- **Description:** eval()
- **Evidence (Line):** `pd.eval("3 not in (1, 2)", engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 576. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1567
- **Description:** eval()
- **Evidence (Line):** `pd.eval("[3] not in (1, 2, [[3]])", engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 577. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1573
- **Description:** eval()
- **Evidence (Line):** `res = pd.eval(expr, engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 578. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1592
- **Description:** eval()
- **Evidence (Line):** `pd.eval(`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 579. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1600
- **Description:** eval()
- **Evidence (Line):** `pd.eval(`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 580. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1614
- **Description:** eval()
- **Evidence (Line):** `pd.eval(ex, parser=parser, engine=engine)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 581. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1617
- **Description:** eval()
- **Evidence (Line):** `pd.eval(ex, parser=parser, engine=engine)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 582. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1621
- **Description:** eval()
- **Evidence (Line):** `def eval(self, *args, **kwargs):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 583. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1623
- **Description:** eval()
- **Evidence (Line):** `return pd.eval(*args, **kwargs)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 584. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1634
- **Description:** eval()
- **Evidence (Line):** `got = self.eval(expr)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 585. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1651
- **Description:** eval()
- **Evidence (Line):** `got = self.eval(expr)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 586. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1663
- **Description:** eval()
- **Evidence (Line):** `df.eval(`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 587. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1680
- **Description:** eval()
- **Evidence (Line):** `df.eval("e = sin(a + b)", engine=engine, parser=parser, inplace=True)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 588. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1705
- **Description:** eval()
- **Evidence (Line):** `df.eval("b = sin(a)", engine=engine, parser=parser, inplace=True)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 589. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1717
- **Description:** eval()
- **Evidence (Line):** `df.eval("mysin(a)", engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 590. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1724
- **Description:** eval()
- **Evidence (Line):** `df.eval("sin(x=a)", engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 591. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1734
- **Description:** eval()
- **Evidence (Line):** `_var_s * 2, pd.eval(e, engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 592. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1740
- **Description:** eval()
- **Evidence (Line):** `pd.eval("x + 1", local_dict=lcls, engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 593. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1748
- **Description:** eval()
- **Evidence (Line):** `pd.eval("x + 1", engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 594. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1757
- **Description:** eval()
- **Evidence (Line):** `pd.eval("x + 1", engine=engine, parser=parser, local_dict={})`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 595. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1764
- **Description:** eval()
- **Evidence (Line):** `pd.eval(e, engine=engine, parser=parser, global_dict={})`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 596. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1771
- **Description:** eval()
- **Evidence (Line):** `pd.eval("x + y", local_dict={"x": 1, "y": 2}, engine="asdf")`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 597. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1807
- **Description:** eval()
- **Evidence (Line):** `pd.eval("x + y", local_dict={"x": 1, "y": 2}, parser="asdf")`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 598. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1832
- **Description:** eval()
- **Evidence (Line):** `pd.eval(e, engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 599. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1839
- **Description:** eval()
- **Evidence (Line):** `pd.eval(e, engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 600. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1848
- **Description:** eval()
- **Evidence (Line):** `pd.eval(express, engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 601. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1851
- **Description:** eval()
- **Evidence (Line):** `pd.eval(express, engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 602. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1859
- **Description:** eval()
- **Evidence (Line):** `pd.eval("sin + dotted_line", engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 603. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1861
- **Description:** eval()
- **Evidence (Line):** `res = pd.eval("sin + dotted_line", engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 604. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1868
- **Description:** eval()
- **Evidence (Line):** `pd.eval("1 + 2", resolvers=cannot_resolve, engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 605. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1874
- **Description:** eval()
- **Evidence (Line):** `pd.eval("", engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 606. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1879
- **Description:** eval()
- **Evidence (Line):** `pd.eval("1 + 1; 2 + 2", engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 607. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1901
- **Description:** eval()
- **Evidence (Line):** `pd.eval(ex, engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 608. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1913
- **Description:** eval()
- **Evidence (Line):** `result = df.eval(f"A == {other}")`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 609. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1925
- **Description:** eval()
- **Evidence (Line):** `result = pd.eval(s, engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 610. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:1989
- **Description:** eval()
- **Evidence (Line):** `df.eval("A = B + C", inplace=True)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 611. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/computation/test_eval.py:2006
- **Description:** eval()
- **Evidence (Line):** `pd.eval("2+2", inplace=value)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 612. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/copy_view/test_astype.py:141
- **Description:** pickle load/dumps
- **Evidence (Line):** `base_copy = pickle.loads(pickle.dumps(base))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 613. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/copy_view/test_astype.py:150
- **Description:** pickle load/dumps
- **Evidence (Line):** `base_copy = pickle.loads(pickle.dumps(base))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 614. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/copy_view/test_astype.py:159
- **Description:** pickle load/dumps
- **Evidence (Line):** `base_copy = pickle.loads(pickle.dumps(base))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 615. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/copy_view/test_methods.py:2026
- **Description:** eval()
- **Evidence (Line):** `def test_eval(using_copy_on_write):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 616. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/copy_view/test_methods.py:2030
- **Description:** eval()
- **Evidence (Line):** `result = df.eval("c = a+b")`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 617. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/copy_view/test_methods.py:2045
- **Description:** eval()
- **Evidence (Line):** `df.eval("c = a+b", inplace=True)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 618. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/extension/test_arrow.py:1514
- **Description:** pickle load/dumps
- **Evidence (Line):** `result = pickle.loads(full_pickled)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 619. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/extension/test_arrow.py:1517
- **Description:** pickle load/dumps
- **Evidence (Line):** `result_sliced = pickle.loads(sliced_pickled)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 620. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/extension/test_arrow.py:2919
- **Description:** pickle load/dumps
- **Evidence (Line):** `result = pickle.loads(pickle.dumps(expected))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 621. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/frame/methods/test_sample.py:176
- **Description:** eval()
- **Evidence (Line):** `result = obj.sample(n=3, random_state=eval(func_str)(arg))`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 622. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/frame/methods/test_sample.py:177
- **Description:** eval()
- **Evidence (Line):** `expected = obj.sample(n=3, random_state=com.random_state(eval(func_str)(arg)))`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 623. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/frame/test_query_eval.py:60
- **Description:** eval()
- **Evidence (Line):** `result = df.eval("A+1")`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 624. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/frame/test_query_eval.py:66
- **Description:** eval()
- **Evidence (Line):** `result = df.eval("A+1", engine=None)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 625. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/frame/test_query_eval.py:72
- **Description:** eval()
- **Evidence (Line):** `result = df.eval("A+1", engine="python")`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 626. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/frame/test_query_eval.py:79
- **Description:** eval()
- **Evidence (Line):** `result = df.eval("A+1", engine="numexpr")`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 627. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/frame/test_query_eval.py:90
- **Description:** eval()
- **Evidence (Line):** `df.eval("A+1", engine="numexpr")`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 628. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/frame/test_query_eval.py:117
- **Description:** eval()
- **Evidence (Line):** `expected = eval(f"base {op_str} df")`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 629. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/frame/test_query_eval.py:120
- **Description:** eval()
- **Evidence (Line):** `result = eval(f"m {op_str} df")`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 630. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/frame/test_query_eval.py:168
- **Description:** eval()
- **Evidence (Line):** `assert df.eval("a + b", resolvers=[dict1, dict2]) == dict1["a"] + dict2["b"]`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 631. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/frame/test_query_eval.py:169
- **Description:** eval()
- **Evidence (Line):** `assert pd.eval("a + b", resolvers=[dict1, dict2]) == dict1["a"] + dict2["b"]`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 632. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/frame/test_query_eval.py:179
- **Description:** eval()
- **Evidence (Line):** `result = df.eval("a + b * c", resolvers=[dict1])`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 633. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/frame/test_query_eval.py:187
- **Description:** eval()
- **Evidence (Line):** `res = df.eval("c = ((a1 == 'Y') & True)")`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 634. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/frame/test_query_eval.py:191
- **Description:** eval()
- **Evidence (Line):** `def test_extension_array_eval(self, engine, parser, request):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 635. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/frame/test_query_eval.py:199
- **Description:** eval()
- **Evidence (Line):** `result = df.eval("a / b", engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 636. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/frame/test_query_eval.py:203
- **Description:** eval()
- **Evidence (Line):** `def test_complex_eval(self, engine, parser):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 637. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/frame/test_query_eval.py:206
- **Description:** eval()
- **Evidence (Line):** `result = df.eval("a/b", engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 638. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/frame/test_query_eval.py:620
- **Description:** eval()
- **Evidence (Line):** `result = pd.eval("df[df > 0 and df2 > 0]", engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 639. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/frame/test_query_eval.py:623
- **Description:** eval()
- **Evidence (Line):** `result = pd.eval(`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 640. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/frame/test_query_eval.py:629
- **Description:** eval()
- **Evidence (Line):** `result = pd.eval("df[(df>0) & (df2>0)]", engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 641. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/frame/test_query_eval.py:871
- **Description:** eval()
- **Evidence (Line):** `result = pd.eval("x + 1", engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 642. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/frame/test_query_eval.py:886
- **Description:** eval()
- **Evidence (Line):** `result = pd.eval("df[(df > 0) & (df2 > 0)]", engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 643. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/frame/test_query_eval.py:890
- **Description:** eval()
- **Evidence (Line):** `result = pd.eval(`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 644. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/frame/test_query_eval.py:1176
- **Description:** eval()
- **Evidence (Line):** `res = frame.eval("a + b", engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 645. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/frame/test_query_eval.py:1181
- **Description:** eval()
- **Evidence (Line):** `res = frame.eval("a[a < 1] + b", engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 646. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/frame/test_query_eval.py:1191
- **Description:** eval()
- **Evidence (Line):** `df.eval(f"a {op} b", engine=engine, parser=parser)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 647. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/frame/test_query_eval.py:1240
- **Description:** eval()
- **Evidence (Line):** `res = df.eval("A + `B B`")`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 648. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/frame/test_query_eval.py:1245
- **Description:** eval()
- **Evidence (Line):** `res = df.eval("`B B` + `C C`")`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 649. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/frame/test_query_eval.py:1250
- **Description:** eval()
- **Evidence (Line):** `res = df.eval("`C_C` + A")`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 650. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/frame/test_query_eval.py:1255
- **Description:** eval()
- **Evidence (Line):** `res = df.eval("C_C + `C C`")`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 651. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/frame/test_query_eval.py:1260
- **Description:** eval()
- **Evidence (Line):** `res = df.eval("A + `D_D D`")`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 652. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/frame/test_query_eval.py:1265
- **Description:** eval()
- **Evidence (Line):** `res = df.eval("A + `C_C`")`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 653. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/frame/test_query_eval.py:1270
- **Description:** eval()
- **Evidence (Line):** `res = df.eval("`E.E` + `F-F` - A")`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 654. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/frame/test_query_eval.py:1275
- **Description:** eval()
- **Evidence (Line):** `res = df.eval("A + `1e1`")`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 655. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/frame/test_query_eval.py:1280
- **Description:** eval()
- **Evidence (Line):** `res = df.eval("A + `def`")`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 656. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/frame/test_query_eval.py:1305
- **Description:** eval()
- **Evidence (Line):** `res = df.eval("` A` + `  `")`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 657. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/frame/test_query_eval.py:1317
- **Description:** eval()
- **Evidence (Line):** `df.eval("@pd.thing")`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 658. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/frame/test_query_eval.py:1347
- **Description:** eval()
- **Evidence (Line):** `df.eval("@func()")`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 659. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/frame/test_query_eval.py:1350
- **Description:** eval()
- **Evidence (Line):** `df.eval("@funcs[0]()")`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 660. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/frame/test_query_eval.py:1353
- **Description:** eval()
- **Evidence (Line):** `df.eval("@funcs[0].__call__()")`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 661. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/frame/test_query_eval.py:1362
- **Description:** eval()
- **Evidence (Line):** `result = df.eval("c = b - a")`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 662. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/frame/test_query_eval.py:1375
- **Description:** eval()
- **Evidence (Line):** `result = df.eval("c = b - 1")`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 663. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/frame/test_query_eval.py:1386
- **Description:** eval()
- **Evidence (Line):** `result = df.eval("c = 2 - 1")`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 664. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/generic/test_finalize.py:449
- **Description:** eval()
- **Evidence (Line):** `result = df.eval("A + 1", engine="numexpr")`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 665. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/indexes/base_class/test_formats.py:16
- **Description:** eval()
- **Evidence (Line):** `res = eval(repr(idx))`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 666. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/indexes/categorical/test_category.py:200
- **Description:** eval()
- **Evidence (Line):** `tm.assert_index_equal(eval(repr(ci)), ci, exact=True)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 667. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/indexes/multi/test_formats.py:64
- **Description:** eval()
- **Evidence (Line):** `eval(repr(mi))`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 668. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/indexes/numeric/test_numeric.py:43
- **Description:** eval()
- **Evidence (Line):** `tm.assert_index_equal(eval(repr(index)), index, exact=True)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 669. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/indexes/ranges/test_range.py:63
- **Description:** eval()
- **Evidence (Line):** `result = eval(result)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 670. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/indexes/ranges/test_range.py:71
- **Description:** eval()
- **Evidence (Line):** `result = eval(result)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 671. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/indexes/ranges/test_range.py:398
- **Description:** eval()
- **Evidence (Line):** `tm.assert_index_equal(eval(repr(index)), index)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 672. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/indexes/test_old_base.py:238
- **Description:** eval()
- **Evidence (Line):** `tm.assert_index_equal(eval(repr(idx)), idx)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 673. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/io/test_pickle.py:171
- **Description:** pickle load/dumps
- **Evidence (Line):** `return pickle.load(fh)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 674. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/io/test_pickle.py:643
- **Description:** pickle load/dumps
- **Evidence (Line):** `df = pickle.load(fd)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 675. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/io/xml/test_to_xml.py:1105
- **Description:** eval()
- **Evidence (Line):** `def test_incorrect_xsl_eval(geom_df):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 676. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/io/xml/test_xml.py:1272
- **Description:** eval()
- **Evidence (Line):** `def test_incorrect_xsl_eval(kml_cta_rail_lines):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 677. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/libs/test_lib.py:295
- **Description:** pickle load/dumps
- **Evidence (Line):** `arr = pickle.loads(pickle.dumps(arr))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 678. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/scalar/test_na_scalar.py:298
- **Description:** pickle load/dumps
- **Evidence (Line):** `result = pickle.loads(pickle.dumps(NA))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 679. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/scalar/timestamp/test_constructors.py:661
- **Description:** eval()
- **Evidence (Line):** `assert result == eval(repr(result))`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 680. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/scalar/timestamp/test_constructors.py:669
- **Description:** eval()
- **Evidence (Line):** `assert result == eval(repr(result))`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 681. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/scalar/timestamp/test_constructors.py:678
- **Description:** eval()
- **Evidence (Line):** `assert result == eval(repr(result))`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 682. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/scalar/timestamp/test_constructors.py:686
- **Description:** eval()
- **Evidence (Line):** `assert result == eval(repr(result))`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 683. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/scalar/timestamp/test_formats.py:110
- **Description:** eval()
- **Evidence (Line):** `assert date_only == eval(repr(date_only))`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 684. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/scalar/timestamp/test_formats.py:116
- **Description:** eval()
- **Evidence (Line):** `assert date_tz == eval(repr(date_tz))`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 685. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/scalar/timestamp/test_formats.py:126
- **Description:** eval()
- **Evidence (Line):** `assert date_with_utc_offset == eval(expr)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 686. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/scalar/timestamp/test_formats.py:163
- **Description:** eval()
- **Evidence (Line):** `assert eval(repr(z)) == z`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 687. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/test_downstream.py:129
- **Description:** pickle load/dumps
- **Evidence (Line):** `"pickle.loads(pickle.dumps(pd.date_range('2021-01-01', periods=1)))"`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 688. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/test_downstream.py:183
- **Description:** yaml.load (unsafe)
- **Evidence (Line):** `loaded = yaml.load(dumped, Loader=yaml.Loader)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 689. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/test_downstream.py:186
- **Description:** yaml.load (unsafe)
- **Evidence (Line):** `loaded2 = yaml.load(dumped, Loader=yaml.UnsafeLoader)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 690. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pip/_internal/commands/configuration.py:248
- **Description:** subprocess with shell=True
- **Evidence (Line):** `subprocess.check_call(f'{editor} "{fname}"', shell=True)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 691. [Critical] Hardcoded Secret
- **File:** .pytest_venv/lib/python3.11/site-packages/pip/_internal/network/auth.py:486
- **Description:** Password Assignment detected
- **Evidence (Line):** `password = ask_password("Password: ")`
- **Exploit Scenario:** Attacker could extract credentials and gain unauthorized access
- **Fix Recommendation:** Move secrets to environment variables or secret manager, rotate compromised credentials
- **Confidence:** High

### 692. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pip/_vendor/distlib/scripts.py:165
- **Description:** exec()
- **Evidence (Line):** `# shebang, or else using os.exec() to run the entry script will`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 693. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pip/_vendor/packaging/_parser.py:372
- **Description:** eval()
- **Evidence (Line):** `value = ast.literal_eval(python_str)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 694. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/pip/_vendor/packaging/_structures.py:10
- **Description:** pickle load/dumps
- **Evidence (Line):** `stand-in classes so that ``pickle.loads()`` can resolve those references.`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 695. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pip/_vendor/pkg_resources/__init__.py:1714
- **Description:** exec()
- **Evidence (Line):** `exec(code, namespace, namespace)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 696. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pip/_vendor/pkg_resources/__init__.py:1725
- **Description:** exec()
- **Evidence (Line):** `exec(script_code, namespace, namespace)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 697. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pip/_vendor/pygments/formatters/__init__.py:91
- **Description:** eval()
- **Evidence (Line):** `this method is equivalent to running ``eval()`` on the input file. The formatter is`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 698. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pip/_vendor/pygments/formatters/__init__.py:103
- **Description:** exec()
- **Evidence (Line):** `exec(f.read(), custom_namespace)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 699. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pip/_vendor/pygments/lexers/__init__.py:154
- **Description:** exec()
- **Evidence (Line):** `exec(f.read(), custom_namespace)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 700. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pip/_vendor/rich/markup.py:190
- **Description:** eval()
- **Evidence (Line):** `meta_params = literal_eval(parameters)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 701. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/labextension/static/0.03957c7088ad5ed0.js:3526
- **Description:** eval()
- **Evidence (Line):** ``),w=c.createShader(c.FRAGMENT_SHADER);if(c.isContextLost()){this.failedToCreate=!0;return}c.shaderS`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 702. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/labextension/static/0.03957c7088ad5ed0.js:1
- **Description:** exec()
- **Evidence (Line):** `(self.rspackChunkjupyterlab_plotly=self.rspackChunkjupyterlab_plotly||[]).push([[0],{2(t,r,i){"use s`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 703. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/labextension/static/0.03957c7088ad5ed0.js:4
- **Description:** exec()
- **Evidence (Line):** ``)},a},t.csv=t.dsv(",","text/csv"),t.tsv=t.dsv("	","text/tab-separated-values");var te,tt,tr,ti,tn=t`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 704. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/labextension/static/0.03957c7088ad5ed0.js:4
- **Description:** exec()
- **Evidence (Line):** ``)},a},t.csv=t.dsv(",","text/csv"),t.tsv=t.dsv("	","text/tab-separated-values");var te,tt,tr,ti,tn=t`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 705. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/labextension/static/0.03957c7088ad5ed0.js:4
- **Description:** exec()
- **Evidence (Line):** ``)},a},t.csv=t.dsv(",","text/csv"),t.tsv=t.dsv("	","text/tab-separated-values");var te,tt,tr,ti,tn=t`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 706. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/labextension/static/0.03957c7088ad5ed0.js:4
- **Description:** exec()
- **Evidence (Line):** ``)},a},t.csv=t.dsv(",","text/csv"),t.tsv=t.dsv("	","text/tab-separated-values");var te,tt,tr,ti,tn=t`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 707. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/labextension/static/0.03957c7088ad5ed0.js:4
- **Description:** exec()
- **Evidence (Line):** ``)},a},t.csv=t.dsv(",","text/csv"),t.tsv=t.dsv("	","text/tab-separated-values");var te,tt,tr,ti,tn=t`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 708. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/labextension/static/0.03957c7088ad5ed0.js:4
- **Description:** exec()
- **Evidence (Line):** ``)},a},t.csv=t.dsv(",","text/csv"),t.tsv=t.dsv("	","text/tab-separated-values");var te,tt,tr,ti,tn=t`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 709. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/labextension/static/0.03957c7088ad5ed0.js:4
- **Description:** exec()
- **Evidence (Line):** ``)},a},t.csv=t.dsv(",","text/csv"),t.tsv=t.dsv("	","text/tab-separated-values");var te,tt,tr,ti,tn=t`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 710. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/labextension/static/0.03957c7088ad5ed0.js:4
- **Description:** exec()
- **Evidence (Line):** ``)},a},t.csv=t.dsv(",","text/csv"),t.tsv=t.dsv("	","text/tab-separated-values");var te,tt,tr,ti,tn=t`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 711. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/labextension/static/0.03957c7088ad5ed0.js:4
- **Description:** exec()
- **Evidence (Line):** ``)},a},t.csv=t.dsv(",","text/csv"),t.tsv=t.dsv("	","text/tab-separated-values");var te,tt,tr,ti,tn=t`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 712. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/labextension/static/0.03957c7088ad5ed0.js:4
- **Description:** exec()
- **Evidence (Line):** ``)},a},t.csv=t.dsv(",","text/csv"),t.tsv=t.dsv("	","text/tab-separated-values");var te,tt,tr,ti,tn=t`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 713. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/labextension/static/0.03957c7088ad5ed0.js:4
- **Description:** exec()
- **Evidence (Line):** ``)},a},t.csv=t.dsv(",","text/csv"),t.tsv=t.dsv("	","text/tab-separated-values");var te,tt,tr,ti,tn=t`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 714. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/labextension/static/0.03957c7088ad5ed0.js:4
- **Description:** exec()
- **Evidence (Line):** ``)},a},t.csv=t.dsv(",","text/csv"),t.tsv=t.dsv("	","text/tab-separated-values");var te,tt,tr,ti,tn=t`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 715. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/labextension/static/0.03957c7088ad5ed0.js:4
- **Description:** exec()
- **Evidence (Line):** ``)},a},t.csv=t.dsv(",","text/csv"),t.tsv=t.dsv("	","text/tab-separated-values");var te,tt,tr,ti,tn=t`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 716. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/labextension/static/0.03957c7088ad5ed0.js:4
- **Description:** exec()
- **Evidence (Line):** ``)},a},t.csv=t.dsv(",","text/csv"),t.tsv=t.dsv("	","text/tab-separated-values");var te,tt,tr,ti,tn=t`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 717. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/labextension/static/0.03957c7088ad5ed0.js:4
- **Description:** exec()
- **Evidence (Line):** ``)},a},t.csv=t.dsv(",","text/csv"),t.tsv=t.dsv("	","text/tab-separated-values");var te,tt,tr,ti,tn=t`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 718. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/labextension/static/0.03957c7088ad5ed0.js:4
- **Description:** exec()
- **Evidence (Line):** ``)},a},t.csv=t.dsv(",","text/csv"),t.tsv=t.dsv("	","text/tab-separated-values");var te,tt,tr,ti,tn=t`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 719. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/labextension/static/0.03957c7088ad5ed0.js:4
- **Description:** exec()
- **Evidence (Line):** ``)},a},t.csv=t.dsv(",","text/csv"),t.tsv=t.dsv("	","text/tab-separated-values");var te,tt,tr,ti,tn=t`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 720. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/labextension/static/0.03957c7088ad5ed0.js:4
- **Description:** exec()
- **Evidence (Line):** ``)},a},t.csv=t.dsv(",","text/csv"),t.tsv=t.dsv("	","text/tab-separated-values");var te,tt,tr,ti,tn=t`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 721. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/labextension/static/0.03957c7088ad5ed0.js:4
- **Description:** exec()
- **Evidence (Line):** ``)},a},t.csv=t.dsv(",","text/csv"),t.tsv=t.dsv("	","text/tab-separated-values");var te,tt,tr,ti,tn=t`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 722. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/labextension/static/0.03957c7088ad5ed0.js:4
- **Description:** exec()
- **Evidence (Line):** ``)},a},t.csv=t.dsv(",","text/csv"),t.tsv=t.dsv("	","text/tab-separated-values");var te,tt,tr,ti,tn=t`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 723. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/labextension/static/0.03957c7088ad5ed0.js:4
- **Description:** exec()
- **Evidence (Line):** ``)},a},t.csv=t.dsv(",","text/csv"),t.tsv=t.dsv("	","text/tab-separated-values");var te,tt,tr,ti,tn=t`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 724. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/labextension/static/0.03957c7088ad5ed0.js:4
- **Description:** exec()
- **Evidence (Line):** ``)},a},t.csv=t.dsv(",","text/csv"),t.tsv=t.dsv("	","text/tab-separated-values");var te,tt,tr,ti,tn=t`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 725. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/labextension/static/0.03957c7088ad5ed0.js:4
- **Description:** exec()
- **Evidence (Line):** ``)},a},t.csv=t.dsv(",","text/csv"),t.tsv=t.dsv("	","text/tab-separated-values");var te,tt,tr,ti,tn=t`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 726. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/labextension/static/0.03957c7088ad5ed0.js:4
- **Description:** exec()
- **Evidence (Line):** ``)},a},t.csv=t.dsv(",","text/csv"),t.tsv=t.dsv("	","text/tab-separated-values");var te,tt,tr,ti,tn=t`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 727. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/labextension/static/0.03957c7088ad5ed0.js:4
- **Description:** exec()
- **Evidence (Line):** ``)},a},t.csv=t.dsv(",","text/csv"),t.tsv=t.dsv("	","text/tab-separated-values");var te,tt,tr,ti,tn=t`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 728. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/labextension/static/0.03957c7088ad5ed0.js:4
- **Description:** exec()
- **Evidence (Line):** ``)},a},t.csv=t.dsv(",","text/csv"),t.tsv=t.dsv("	","text/tab-separated-values");var te,tt,tr,ti,tn=t`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 729. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/labextension/static/0.03957c7088ad5ed0.js:4
- **Description:** exec()
- **Evidence (Line):** ``)},a},t.csv=t.dsv(",","text/csv"),t.tsv=t.dsv("	","text/tab-separated-values");var te,tt,tr,ti,tn=t`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 730. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/labextension/static/0.03957c7088ad5ed0.js:4
- **Description:** exec()
- **Evidence (Line):** ``)},a},t.csv=t.dsv(",","text/csv"),t.tsv=t.dsv("	","text/tab-separated-values");var te,tt,tr,ti,tn=t`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 731. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/labextension/static/0.03957c7088ad5ed0.js:9
- **Description:** exec()
- **Evidence (Line):** ``+l.year}return I(r,t,n,o)};var F=3*c;t.incrementMonth=function(t,r,i){i=_(i)&&i;var n=s(t,c);if(t=M`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 732. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/labextension/static/0.03957c7088ad5ed0.js:85
- **Description:** exec()
- **Evidence (Line):** ``))}throw a}},T.match=function t(r,i,n){O(r,i,n,t,"match")},T.doesNotMatch=function t(r,i,n){O(r,i,n`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 733. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/labextension/static/0.03957c7088ad5ed0.js:89
- **Description:** exec()
- **Evidence (Line):** `children sum =`,a].join(" "))}t.value=i})}else!function t(r,i,n){var a=0,o=r.children;if(o){for(var `
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 734. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/labextension/static/0.03957c7088ad5ed0.js:89
- **Description:** exec()
- **Evidence (Line):** `children sum =`,a].join(" "))}t.value=i})}else!function t(r,i,n){var a=0,o=r.children;if(o){for(var `
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 735. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/labextension/static/0.03957c7088ad5ed0.js:89
- **Description:** exec()
- **Evidence (Line):** `children sum =`,a].join(" "))}t.value=i})}else!function t(r,i,n){var a=0,o=r.children;if(o){for(var `
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 736. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/labextension/static/0.03957c7088ad5ed0.js:89
- **Description:** exec()
- **Evidence (Line):** `children sum =`,a].join(" "))}t.value=i})}else!function t(r,i,n){var a=0,o=r.children;if(o){for(var `
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 737. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/labextension/static/0.03957c7088ad5ed0.js:89
- **Description:** exec()
- **Evidence (Line):** `children sum =`,a].join(" "))}t.value=i})}else!function t(r,i,n){var a=0,o=r.children;if(o){for(var `
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 738. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/labextension/static/0.03957c7088ad5ed0.js:89
- **Description:** exec()
- **Evidence (Line):** `children sum =`,a].join(" "))}t.value=i})}else!function t(r,i,n){var a=0,o=r.children;if(o){for(var `
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 739. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/labextension/static/0.03957c7088ad5ed0.js:89
- **Description:** exec()
- **Evidence (Line):** `children sum =`,a].join(" "))}t.value=i})}else!function t(r,i,n){var a=0,o=r.children;if(o){for(var `
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 740. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/labextension/static/0.03957c7088ad5ed0.js:89
- **Description:** exec()
- **Evidence (Line):** `children sum =`,a].join(" "))}t.value=i})}else!function t(r,i,n){var a=0,o=r.children;if(o){for(var `
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 741. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/labextension/static/0.03957c7088ad5ed0.js:89
- **Description:** exec()
- **Evidence (Line):** `children sum =`,a].join(" "))}t.value=i})}else!function t(r,i,n){var a=0,o=r.children;if(o){for(var `
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 742. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/labextension/static/0.03957c7088ad5ed0.js:1994
- **Description:** exec()
- **Evidence (Line):** ``)&&"\\"!==i?(k(p.join("")),f=999,h):(p.push(r),i=r,h+1)}function E(t){for(var r,i,n=0;;){if(r=a.ind`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 743. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/labextension/static/0.03957c7088ad5ed0.js:1994
- **Description:** exec()
- **Evidence (Line):** ``)&&"\\"!==i?(k(p.join("")),f=999,h):(p.push(r),i=r,h+1)}function E(t){for(var r,i,n=0;;){if(r=a.ind`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 744. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/labextension/static/0.03957c7088ad5ed0.js:1994
- **Description:** exec()
- **Evidence (Line):** ``)&&"\\"!==i?(k(p.join("")),f=999,h):(p.push(r),i=r,h+1)}function E(t){for(var r,i,n=0;;){if(r=a.ind`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 745. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/labextension/static/0.03957c7088ad5ed0.js:1994
- **Description:** exec()
- **Evidence (Line):** ``)&&"\\"!==i?(k(p.join("")),f=999,h):(p.push(r),i=r,h+1)}function E(t){for(var r,i,n=0;;){if(r=a.ind`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 746. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/labextension/static/0.03957c7088ad5ed0.js:1994
- **Description:** exec()
- **Evidence (Line):** ``)&&"\\"!==i?(k(p.join("")),f=999,h):(p.push(r),i=r,h+1)}function E(t){for(var r,i,n=0;;){if(r=a.ind`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 747. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/labextension/static/0.03957c7088ad5ed0.js:1994
- **Description:** exec()
- **Evidence (Line):** ``)&&"\\"!==i?(k(p.join("")),f=999,h):(p.push(r),i=r,h+1)}function E(t){for(var r,i,n=0;;){if(r=a.ind`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 748. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/labextension/static/0.03957c7088ad5ed0.js:1997
- **Description:** exec()
- **Evidence (Line):** ``;for(_=0,w=x*m,T=n,M="",b=0;b<E.length;++b){var D=b+A<h.length?h[b+A]:h[h.length-1];l===D?M+=E[b]:(`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 749. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/labextension/static/0.03957c7088ad5ed0.js:2618
- **Description:** exec()
- **Evidence (Line):** ``,uniforms:{range:t.prop("range"),lineWidth:t.prop("lineWidth"),capSize:t.prop("capSize"),opacity:t.`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 750. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/labextension/static/0.03957c7088ad5ed0.js:2618
- **Description:** exec()
- **Evidence (Line):** ``,uniforms:{range:t.prop("range"),lineWidth:t.prop("lineWidth"),capSize:t.prop("capSize"),opacity:t.`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 751. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/labextension/static/0.03957c7088ad5ed0.js:2710
- **Description:** exec()
- **Evidence (Line):** ``)),x.push({type:"Feature",geometry:{type:"Point",coordinates:_.lonlat},properties:{symbol:h(b),angl`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 752. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/labextension/static/0.03957c7088ad5ed0.js:2762
- **Description:** exec()
- **Evidence (Line):** `${l.shaderPreludeCode.vertexSource}`,define:l.shaderDefine},defaultProjectionData:c,getProjectionDat`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 753. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/labextension/static/0.03957c7088ad5ed0.js:2762
- **Description:** exec()
- **Evidence (Line):** `${l.shaderPreludeCode.vertexSource}`,define:l.shaderDefine},defaultProjectionData:c,getProjectionDat`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 754. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/labextension/static/0.03957c7088ad5ed0.js:3526
- **Description:** exec()
- **Evidence (Line):** ``),w=c.createShader(c.FRAGMENT_SHADER);if(c.isContextLost()){this.failedToCreate=!0;return}c.shaderS`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 755. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/labextension/static/0.03957c7088ad5ed0.js:3526
- **Description:** exec()
- **Evidence (Line):** ``),w=c.createShader(c.FRAGMENT_SHADER);if(c.isContextLost()){this.failedToCreate=!0;return}c.shaderS`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 756. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/labextension/static/0.03957c7088ad5ed0.js:3526
- **Description:** exec()
- **Evidence (Line):** ``),w=c.createShader(c.FRAGMENT_SHADER);if(c.isContextLost()){this.failedToCreate=!0;return}c.shaderS`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 757. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/labextension/static/0.03957c7088ad5ed0.js:3526
- **Description:** exec()
- **Evidence (Line):** ``),w=c.createShader(c.FRAGMENT_SHADER);if(c.isContextLost()){this.failedToCreate=!0;return}c.shaderS`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 758. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/labextension/static/remoteEntry.1dda3c8f6e5cd2a3.js:1
- **Description:** exec()
- **Evidence (Line):** `var _JUPYTERLAB;(()=>{"use strict";var e,n,r,t,o,i,u,a,c,l,f,s,p,h,d,v,g,y,m,b,S,j,w,E,k,x,T,P,A,C,O`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 759. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/labextension/static/remoteEntry.1dda3c8f6e5cd2a3.js:1
- **Description:** exec()
- **Evidence (Line):** `var _JUPYTERLAB;(()=>{"use strict";var e,n,r,t,o,i,u,a,c,l,f,s,p,h,d,v,g,y,m,b,S,j,w,E,k,x,T,P,A,C,O`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 760. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/labextension/static/remoteEntry.1dda3c8f6e5cd2a3.js:1
- **Description:** exec()
- **Evidence (Line):** `var _JUPYTERLAB;(()=>{"use strict";var e,n,r,t,o,i,u,a,c,l,f,s,p,h,d,v,g,y,m,b,S,j,w,E,k,x,T,P,A,C,O`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 761. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/labextension/static/remoteEntry.1dda3c8f6e5cd2a3.js:1
- **Description:** exec()
- **Evidence (Line):** `var _JUPYTERLAB;(()=>{"use strict";var e,n,r,t,o,i,u,a,c,l,f,s,p,h,d,v,g,y,m,b,S,j,w,E,k,x,T,P,A,C,O`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 762. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/plotly.min.js:3540
- **Description:** eval()
- **Evidence (Line):** ``),M=u.createShader(u.FRAGMENT_SHADER);if(u.isContextLost()){this.failedToCreate=!0;return}u.shaderS`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 763. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/plotly.min.js:15
- **Description:** exec()
- **Evidence (Line):** `"use strict";var Plotly=(()=>{var B1t=Object.create;var S6=Object.defineProperty,q1t=Object.definePr`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 764. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/plotly.min.js:18
- **Description:** exec()
- **Evidence (Line):** ``)};function me(He){return He.map(ze).join(z)}function ze(He){return Z.test(He)?'"'+He.replace(/\"/g`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 765. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/plotly.min.js:18
- **Description:** exec()
- **Evidence (Line):** ``)};function me(He){return He.map(ze).join(z)}function ze(He){return Z.test(He)?'"'+He.replace(/\"/g`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 766. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/plotly.min.js:18
- **Description:** exec()
- **Evidence (Line):** ``)};function me(He){return He.map(ze).join(z)}function ze(He){return Z.test(He)?'"'+He.replace(/\"/g`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 767. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/plotly.min.js:18
- **Description:** exec()
- **Evidence (Line):** ``)};function me(He){return He.map(ze).join(z)}function ze(He){return Z.test(He)?'"'+He.replace(/\"/g`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 768. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/plotly.min.js:18
- **Description:** exec()
- **Evidence (Line):** ``)};function me(He){return He.map(ze).join(z)}function ze(He){return Z.test(He)?'"'+He.replace(/\"/g`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 769. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/plotly.min.js:18
- **Description:** exec()
- **Evidence (Line):** ``)};function me(He){return He.map(ze).join(z)}function ze(He){return Z.test(He)?'"'+He.replace(/\"/g`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 770. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/plotly.min.js:18
- **Description:** exec()
- **Evidence (Line):** ``)};function me(He){return He.map(ze).join(z)}function ze(He){return Z.test(He)?'"'+He.replace(/\"/g`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 771. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/plotly.min.js:18
- **Description:** exec()
- **Evidence (Line):** ``)};function me(He){return He.map(ze).join(z)}function ze(He){return Z.test(He)?'"'+He.replace(/\"/g`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 772. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/plotly.min.js:18
- **Description:** exec()
- **Evidence (Line):** ``)};function me(He){return He.map(ze).join(z)}function ze(He){return Z.test(He)?'"'+He.replace(/\"/g`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 773. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/plotly.min.js:18
- **Description:** exec()
- **Evidence (Line):** ``)};function me(He){return He.map(ze).join(z)}function ze(He){return Z.test(He)?'"'+He.replace(/\"/g`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 774. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/plotly.min.js:18
- **Description:** exec()
- **Evidence (Line):** ``)};function me(He){return He.map(ze).join(z)}function ze(He){return Z.test(He)?'"'+He.replace(/\"/g`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 775. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/plotly.min.js:18
- **Description:** exec()
- **Evidence (Line):** ``)};function me(He){return He.map(ze).join(z)}function ze(He){return Z.test(He)?'"'+He.replace(/\"/g`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 776. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/plotly.min.js:18
- **Description:** exec()
- **Evidence (Line):** ``)};function me(He){return He.map(ze).join(z)}function ze(He){return Z.test(He)?'"'+He.replace(/\"/g`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 777. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/plotly.min.js:18
- **Description:** exec()
- **Evidence (Line):** ``)};function me(He){return He.map(ze).join(z)}function ze(He){return Z.test(He)?'"'+He.replace(/\"/g`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 778. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/plotly.min.js:18
- **Description:** exec()
- **Evidence (Line):** ``)};function me(He){return He.map(ze).join(z)}function ze(He){return Z.test(He)?'"'+He.replace(/\"/g`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 779. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/plotly.min.js:18
- **Description:** exec()
- **Evidence (Line):** ``)};function me(He){return He.map(ze).join(z)}function ze(He){return Z.test(He)?'"'+He.replace(/\"/g`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 780. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/plotly.min.js:18
- **Description:** exec()
- **Evidence (Line):** ``)};function me(He){return He.map(ze).join(z)}function ze(He){return Z.test(He)?'"'+He.replace(/\"/g`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 781. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/plotly.min.js:18
- **Description:** exec()
- **Evidence (Line):** ``)};function me(He){return He.map(ze).join(z)}function ze(He){return Z.test(He)?'"'+He.replace(/\"/g`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 782. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/plotly.min.js:18
- **Description:** exec()
- **Evidence (Line):** ``)};function me(He){return He.map(ze).join(z)}function ze(He){return Z.test(He)?'"'+He.replace(/\"/g`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 783. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/plotly.min.js:18
- **Description:** exec()
- **Evidence (Line):** ``)};function me(He){return He.map(ze).join(z)}function ze(He){return Z.test(He)?'"'+He.replace(/\"/g`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 784. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/plotly.min.js:18
- **Description:** exec()
- **Evidence (Line):** ``)};function me(He){return He.map(ze).join(z)}function ze(He){return Z.test(He)?'"'+He.replace(/\"/g`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 785. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/plotly.min.js:18
- **Description:** exec()
- **Evidence (Line):** ``)};function me(He){return He.map(ze).join(z)}function ze(He){return Z.test(He)?'"'+He.replace(/\"/g`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 786. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/plotly.min.js:18
- **Description:** exec()
- **Evidence (Line):** ``)};function me(He){return He.map(ze).join(z)}function ze(He){return Z.test(He)?'"'+He.replace(/\"/g`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 787. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/plotly.min.js:18
- **Description:** exec()
- **Evidence (Line):** ``)};function me(He){return He.map(ze).join(z)}function ze(He){return Z.test(He)?'"'+He.replace(/\"/g`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 788. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/plotly.min.js:18
- **Description:** exec()
- **Evidence (Line):** ``)};function me(He){return He.map(ze).join(z)}function ze(He){return Z.test(He)?'"'+He.replace(/\"/g`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 789. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/plotly.min.js:18
- **Description:** exec()
- **Evidence (Line):** ``)};function me(He){return He.map(ze).join(z)}function ze(He){return Z.test(He)?'"'+He.replace(/\"/g`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 790. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/plotly.min.js:18
- **Description:** exec()
- **Evidence (Line):** ``)};function me(He){return He.map(ze).join(z)}function ze(He){return Z.test(He)?'"'+He.replace(/\"/g`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 791. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/plotly.min.js:18
- **Description:** exec()
- **Evidence (Line):** ``)};function me(He){return He.map(ze).join(z)}function ze(He){return Z.test(He)?'"'+He.replace(/\"/g`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 792. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/plotly.min.js:23
- **Description:** exec()
- **Evidence (Line):** ``+qhe(a.dayMonthYear,e,i,n);return qhe(t,e,i,n)};var Ohe=3*Ih;Sl.incrementMonth=function(e,t,r){r=Dg`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 793. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/plotly.min.js:99
- **Description:** exec()
- **Evidence (Line):** ``));var s=new f2({actual:e,expected:t,message:r,operator:n,stackStartFn:i});throw s.generatedMessage`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 794. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/plotly.min.js:103
- **Description:** exec()
- **Evidence (Line):** `children sum =`,B].join(" "))}R.value=I});break}else YIe(S,t,{branches:t.count.indexOf("branches")!=`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 795. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/plotly.min.js:103
- **Description:** exec()
- **Evidence (Line):** `children sum =`,B].join(" "))}R.value=I});break}else YIe(S,t,{branches:t.count.indexOf("branches")!=`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 796. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/plotly.min.js:103
- **Description:** exec()
- **Evidence (Line):** `children sum =`,B].join(" "))}R.value=I});break}else YIe(S,t,{branches:t.count.indexOf("branches")!=`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 797. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/plotly.min.js:103
- **Description:** exec()
- **Evidence (Line):** `children sum =`,B].join(" "))}R.value=I});break}else YIe(S,t,{branches:t.count.indexOf("branches")!=`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 798. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/plotly.min.js:103
- **Description:** exec()
- **Evidence (Line):** `children sum =`,B].join(" "))}R.value=I});break}else YIe(S,t,{branches:t.count.indexOf("branches")!=`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 799. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/plotly.min.js:103
- **Description:** exec()
- **Evidence (Line):** `children sum =`,B].join(" "))}R.value=I});break}else YIe(S,t,{branches:t.count.indexOf("branches")!=`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 800. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/plotly.min.js:103
- **Description:** exec()
- **Evidence (Line):** `children sum =`,B].join(" "))}R.value=I});break}else YIe(S,t,{branches:t.count.indexOf("branches")!=`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 801. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/plotly.min.js:103
- **Description:** exec()
- **Evidence (Line):** `children sum =`,B].join(" "))}R.value=I});break}else YIe(S,t,{branches:t.count.indexOf("branches")!=`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 802. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/plotly.min.js:103
- **Description:** exec()
- **Evidence (Line):** `children sum =`,B].join(" "))}R.value=I});break}else YIe(S,t,{branches:t.count.indexOf("branches")!=`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 803. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/plotly.min.js:2008
- **Description:** exec()
- **Evidence (Line):** ``)&&R!=="\\"?(Q(L.join("")),F=h,k):(L.push(P),R=P,k+1)}function he(){return be()}function oe(){retur`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 804. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/plotly.min.js:2008
- **Description:** exec()
- **Evidence (Line):** ``)&&R!=="\\"?(Q(L.join("")),F=h,k):(L.push(P),R=P,k+1)}function he(){return be()}function oe(){retur`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 805. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/plotly.min.js:2008
- **Description:** exec()
- **Evidence (Line):** ``)&&R!=="\\"?(Q(L.join("")),F=h,k):(L.push(P),R=P,k+1)}function he(){return be()}function oe(){retur`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 806. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/plotly.min.js:2008
- **Description:** exec()
- **Evidence (Line):** ``)&&R!=="\\"?(Q(L.join("")),F=h,k):(L.push(P),R=P,k+1)}function he(){return be()}function oe(){retur`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 807. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/plotly.min.js:2008
- **Description:** exec()
- **Evidence (Line):** ``)&&R!=="\\"?(Q(L.join("")),F=h,k):(L.push(P),R=P,k+1)}function he(){return be()}function oe(){retur`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 808. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/plotly.min.js:2008
- **Description:** exec()
- **Evidence (Line):** ``)&&R!=="\\"?(Q(L.join("")),F=h,k):(L.push(P),R=P,k+1)}function he(){return be()}function oe(){retur`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 809. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/plotly.min.js:2011
- **Description:** exec()
- **Evidence (Line):** ``;for(Q=0,te=H*q,ce=R,Ce="",G=0;G<Ee.length;++G){var ye=G+ge<$.length?$[G+ge]:$[$.length-1];B===ye?C`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 810. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/plotly.min.js:2632
- **Description:** exec()
- **Evidence (Line):** ``,uniforms:{range:e.prop("range"),lineWidth:e.prop("lineWidth"),capSize:e.prop("capSize"),opacity:e.`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 811. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/plotly.min.js:2632
- **Description:** exec()
- **Evidence (Line):** ``,uniforms:{range:e.prop("range"),lineWidth:e.prop("lineWidth"),capSize:e.prop("capSize"),opacity:e.`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 812. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/plotly.min.js:2724
- **Description:** exec()
- **Evidence (Line):** ``)),c.push({type:"Feature",geometry:{type:"Point",coordinates:h.lonlat},properties:{symbol:s(f),angl`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 813. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/plotly.min.js:2725
- **Description:** exec()
- **Evidence (Line):** `Use an identity property function instead: \`{ "type": "identity", "property": ${JSON.stringify(f[1]`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 814. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/plotly.min.js:2725
- **Description:** exec()
- **Evidence (Line):** `Use an identity property function instead: \`{ "type": "identity", "property": ${JSON.stringify(f[1]`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 815. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/plotly.min.js:3540
- **Description:** exec()
- **Evidence (Line):** ``),M=u.createShader(u.FRAGMENT_SHADER);if(u.isContextLost()){this.failedToCreate=!0;return}u.shaderS`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 816. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/plotly.min.js:3540
- **Description:** exec()
- **Evidence (Line):** ``),M=u.createShader(u.FRAGMENT_SHADER);if(u.isContextLost()){this.failedToCreate=!0;return}u.shaderS`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 817. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/plotly.min.js:3540
- **Description:** exec()
- **Evidence (Line):** ``),M=u.createShader(u.FRAGMENT_SHADER);if(u.isContextLost()){this.failedToCreate=!0;return}u.shaderS`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 818. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/widgetbundle.js:3526
- **Description:** eval()
- **Evidence (Line):** ``),E=u.createShader(u.FRAGMENT_SHADER);if(u.isContextLost()){this.failedToCreate=!0;return}u.shaderS`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 819. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/widgetbundle.js:1
- **Description:** exec()
- **Evidence (Line):** `var obe=Object.create;var NG=Object.defineProperty;var sbe=Object.getOwnPropertyDescriptor;var lbe=O`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 820. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/widgetbundle.js:4
- **Description:** exec()
- **Evidence (Line):** ``)};function Qe(Ft){return Ft.map(yt).join(ae)}function yt(Ft){return Te.test(Ft)?'"'+Ft.replace(/\"`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 821. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/widgetbundle.js:4
- **Description:** exec()
- **Evidence (Line):** ``)};function Qe(Ft){return Ft.map(yt).join(ae)}function yt(Ft){return Te.test(Ft)?'"'+Ft.replace(/\"`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 822. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/widgetbundle.js:4
- **Description:** exec()
- **Evidence (Line):** ``)};function Qe(Ft){return Ft.map(yt).join(ae)}function yt(Ft){return Te.test(Ft)?'"'+Ft.replace(/\"`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 823. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/widgetbundle.js:4
- **Description:** exec()
- **Evidence (Line):** ``)};function Qe(Ft){return Ft.map(yt).join(ae)}function yt(Ft){return Te.test(Ft)?'"'+Ft.replace(/\"`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 824. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/widgetbundle.js:4
- **Description:** exec()
- **Evidence (Line):** ``)};function Qe(Ft){return Ft.map(yt).join(ae)}function yt(Ft){return Te.test(Ft)?'"'+Ft.replace(/\"`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 825. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/widgetbundle.js:4
- **Description:** exec()
- **Evidence (Line):** ``)};function Qe(Ft){return Ft.map(yt).join(ae)}function yt(Ft){return Te.test(Ft)?'"'+Ft.replace(/\"`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 826. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/widgetbundle.js:4
- **Description:** exec()
- **Evidence (Line):** ``)};function Qe(Ft){return Ft.map(yt).join(ae)}function yt(Ft){return Te.test(Ft)?'"'+Ft.replace(/\"`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 827. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/widgetbundle.js:4
- **Description:** exec()
- **Evidence (Line):** ``)};function Qe(Ft){return Ft.map(yt).join(ae)}function yt(Ft){return Te.test(Ft)?'"'+Ft.replace(/\"`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 828. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/widgetbundle.js:4
- **Description:** exec()
- **Evidence (Line):** ``)};function Qe(Ft){return Ft.map(yt).join(ae)}function yt(Ft){return Te.test(Ft)?'"'+Ft.replace(/\"`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 829. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/widgetbundle.js:4
- **Description:** exec()
- **Evidence (Line):** ``)};function Qe(Ft){return Ft.map(yt).join(ae)}function yt(Ft){return Te.test(Ft)?'"'+Ft.replace(/\"`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 830. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/widgetbundle.js:4
- **Description:** exec()
- **Evidence (Line):** ``)};function Qe(Ft){return Ft.map(yt).join(ae)}function yt(Ft){return Te.test(Ft)?'"'+Ft.replace(/\"`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 831. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/widgetbundle.js:4
- **Description:** exec()
- **Evidence (Line):** ``)};function Qe(Ft){return Ft.map(yt).join(ae)}function yt(Ft){return Te.test(Ft)?'"'+Ft.replace(/\"`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 832. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/widgetbundle.js:4
- **Description:** exec()
- **Evidence (Line):** ``)};function Qe(Ft){return Ft.map(yt).join(ae)}function yt(Ft){return Te.test(Ft)?'"'+Ft.replace(/\"`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 833. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/widgetbundle.js:4
- **Description:** exec()
- **Evidence (Line):** ``)};function Qe(Ft){return Ft.map(yt).join(ae)}function yt(Ft){return Te.test(Ft)?'"'+Ft.replace(/\"`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 834. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/widgetbundle.js:4
- **Description:** exec()
- **Evidence (Line):** ``)};function Qe(Ft){return Ft.map(yt).join(ae)}function yt(Ft){return Te.test(Ft)?'"'+Ft.replace(/\"`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 835. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/widgetbundle.js:4
- **Description:** exec()
- **Evidence (Line):** ``)};function Qe(Ft){return Ft.map(yt).join(ae)}function yt(Ft){return Te.test(Ft)?'"'+Ft.replace(/\"`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 836. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/widgetbundle.js:4
- **Description:** exec()
- **Evidence (Line):** ``)};function Qe(Ft){return Ft.map(yt).join(ae)}function yt(Ft){return Te.test(Ft)?'"'+Ft.replace(/\"`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 837. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/widgetbundle.js:4
- **Description:** exec()
- **Evidence (Line):** ``)};function Qe(Ft){return Ft.map(yt).join(ae)}function yt(Ft){return Te.test(Ft)?'"'+Ft.replace(/\"`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 838. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/widgetbundle.js:4
- **Description:** exec()
- **Evidence (Line):** ``)};function Qe(Ft){return Ft.map(yt).join(ae)}function yt(Ft){return Te.test(Ft)?'"'+Ft.replace(/\"`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 839. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/widgetbundle.js:4
- **Description:** exec()
- **Evidence (Line):** ``)};function Qe(Ft){return Ft.map(yt).join(ae)}function yt(Ft){return Te.test(Ft)?'"'+Ft.replace(/\"`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 840. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/widgetbundle.js:4
- **Description:** exec()
- **Evidence (Line):** ``)};function Qe(Ft){return Ft.map(yt).join(ae)}function yt(Ft){return Te.test(Ft)?'"'+Ft.replace(/\"`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 841. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/widgetbundle.js:4
- **Description:** exec()
- **Evidence (Line):** ``)};function Qe(Ft){return Ft.map(yt).join(ae)}function yt(Ft){return Te.test(Ft)?'"'+Ft.replace(/\"`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 842. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/widgetbundle.js:4
- **Description:** exec()
- **Evidence (Line):** ``)};function Qe(Ft){return Ft.map(yt).join(ae)}function yt(Ft){return Te.test(Ft)?'"'+Ft.replace(/\"`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 843. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/widgetbundle.js:4
- **Description:** exec()
- **Evidence (Line):** ``)};function Qe(Ft){return Ft.map(yt).join(ae)}function yt(Ft){return Te.test(Ft)?'"'+Ft.replace(/\"`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 844. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/widgetbundle.js:4
- **Description:** exec()
- **Evidence (Line):** ``)};function Qe(Ft){return Ft.map(yt).join(ae)}function yt(Ft){return Te.test(Ft)?'"'+Ft.replace(/\"`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 845. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/widgetbundle.js:4
- **Description:** exec()
- **Evidence (Line):** ``)};function Qe(Ft){return Ft.map(yt).join(ae)}function yt(Ft){return Te.test(Ft)?'"'+Ft.replace(/\"`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 846. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/widgetbundle.js:4
- **Description:** exec()
- **Evidence (Line):** ``)};function Qe(Ft){return Ft.map(yt).join(ae)}function yt(Ft){return Te.test(Ft)?'"'+Ft.replace(/\"`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 847. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/widgetbundle.js:4
- **Description:** exec()
- **Evidence (Line):** ``)};function Qe(Ft){return Ft.map(yt).join(ae)}function yt(Ft){return Te.test(Ft)?'"'+Ft.replace(/\"`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 848. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/widgetbundle.js:9
- **Description:** exec()
- **Evidence (Line):** ``+A(O.dayMonthYear,P,L,B);return A(z,P,L,B)};var R=3*l;e.incrementMonth=function(P,z,I){I=y(I)&&I;va`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 849. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/widgetbundle.js:85
- **Description:** exec()
- **Evidence (Line):** ``));var K=new g({actual:H,expected:W,message:U,operator:j,stackStartFn:V});throw K.generatedMessage=`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 850. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/widgetbundle.js:89
- **Description:** exec()
- **Evidence (Line):** `children sum =`,j].join(" "))}W.value=V});break}else h($,p,{branches:p.count.indexOf("branches")!==-`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 851. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/widgetbundle.js:89
- **Description:** exec()
- **Evidence (Line):** `children sum =`,j].join(" "))}W.value=V});break}else h($,p,{branches:p.count.indexOf("branches")!==-`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 852. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/widgetbundle.js:89
- **Description:** exec()
- **Evidence (Line):** `children sum =`,j].join(" "))}W.value=V});break}else h($,p,{branches:p.count.indexOf("branches")!==-`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 853. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/widgetbundle.js:89
- **Description:** exec()
- **Evidence (Line):** `children sum =`,j].join(" "))}W.value=V});break}else h($,p,{branches:p.count.indexOf("branches")!==-`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 854. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/widgetbundle.js:89
- **Description:** exec()
- **Evidence (Line):** `children sum =`,j].join(" "))}W.value=V});break}else h($,p,{branches:p.count.indexOf("branches")!==-`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 855. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/widgetbundle.js:89
- **Description:** exec()
- **Evidence (Line):** `children sum =`,j].join(" "))}W.value=V});break}else h($,p,{branches:p.count.indexOf("branches")!==-`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 856. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/widgetbundle.js:89
- **Description:** exec()
- **Evidence (Line):** `children sum =`,j].join(" "))}W.value=V});break}else h($,p,{branches:p.count.indexOf("branches")!==-`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 857. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/widgetbundle.js:89
- **Description:** exec()
- **Evidence (Line):** `children sum =`,j].join(" "))}W.value=V});break}else h($,p,{branches:p.count.indexOf("branches")!==-`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 858. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/widgetbundle.js:89
- **Description:** exec()
- **Evidence (Line):** `children sum =`,j].join(" "))}W.value=V});break}else h($,p,{branches:p.count.indexOf("branches")!==-`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 859. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/widgetbundle.js:1994
- **Description:** exec()
- **Evidence (Line):** ``)&&I!=="\\"?(Y(L.join("")),P=p,D):(L.push(z),I=z,D+1)}function fe(){return ue()}function pe(){retur`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 860. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/widgetbundle.js:1994
- **Description:** exec()
- **Evidence (Line):** ``)&&I!=="\\"?(Y(L.join("")),P=p,D):(L.push(z),I=z,D+1)}function fe(){return ue()}function pe(){retur`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 861. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/widgetbundle.js:1994
- **Description:** exec()
- **Evidence (Line):** ``)&&I!=="\\"?(Y(L.join("")),P=p,D):(L.push(z),I=z,D+1)}function fe(){return ue()}function pe(){retur`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 862. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/widgetbundle.js:1994
- **Description:** exec()
- **Evidence (Line):** ``)&&I!=="\\"?(Y(L.join("")),P=p,D):(L.push(z),I=z,D+1)}function fe(){return ue()}function pe(){retur`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 863. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/widgetbundle.js:1994
- **Description:** exec()
- **Evidence (Line):** ``)&&I!=="\\"?(Y(L.join("")),P=p,D):(L.push(z),I=z,D+1)}function fe(){return ue()}function pe(){retur`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 864. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/widgetbundle.js:1994
- **Description:** exec()
- **Evidence (Line):** ``)&&I!=="\\"?(Y(L.join("")),P=p,D):(L.push(z),I=z,D+1)}function fe(){return ue()}function pe(){retur`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 865. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/widgetbundle.js:1997
- **Description:** exec()
- **Evidence (Line):** ``;for(Y=0,K=j*N,re=I,le="",q=0;q<we.length;++q){var xe=q+se<$.length?$[q+se]:$[$.length-1];O===xe?le`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 866. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/widgetbundle.js:2618
- **Description:** exec()
- **Evidence (Line):** ``,uniforms:{range:h.prop("range"),lineWidth:h.prop("lineWidth"),capSize:h.prop("capSize"),opacity:h.`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 867. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/widgetbundle.js:2618
- **Description:** exec()
- **Evidence (Line):** ``,uniforms:{range:h.prop("range"),lineWidth:h.prop("lineWidth"),capSize:h.prop("capSize"),opacity:h.`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 868. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/widgetbundle.js:2710
- **Description:** exec()
- **Evidence (Line):** ``)),P.push({type:"Feature",geometry:{type:"Point",coordinates:I.lonlat},properties:{symbol:F(z),angl`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 869. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/widgetbundle.js:2711
- **Description:** exec()
- **Evidence (Line):** `Use an identity property function instead: \`{ "type": "identity", "property": ${JSON.stringify(f[1]`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 870. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/widgetbundle.js:2711
- **Description:** exec()
- **Evidence (Line):** `Use an identity property function instead: \`{ "type": "identity", "property": ${JSON.stringify(f[1]`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 871. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/widgetbundle.js:3526
- **Description:** exec()
- **Evidence (Line):** ``),E=u.createShader(u.FRAGMENT_SHADER);if(u.isContextLost()){this.failedToCreate=!0;return}u.shaderS`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 872. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/widgetbundle.js:3526
- **Description:** exec()
- **Evidence (Line):** ``),E=u.createShader(u.FRAGMENT_SHADER);if(u.isContextLost()){this.failedToCreate=!0;return}u.shaderS`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 873. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/widgetbundle.js:3526
- **Description:** exec()
- **Evidence (Line):** ``),E=u.createShader(u.FRAGMENT_SHADER);if(u.isContextLost()){this.failedToCreate=!0;return}u.shaderS`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 874. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/widgetbundle.js:3526
- **Description:** exec()
- **Evidence (Line):** ``),E=u.createShader(u.FRAGMENT_SHADER);if(u.isContextLost()){this.failedToCreate=!0;return}u.shaderS`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 875. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/widgetbundle.js:3528
- **Description:** exec()
- **Evidence (Line):** ``)}var iH=ewe;function twe(Q){return function(){return Q}}var fg=twe;var rwe=(function(){try{var Q=o`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 876. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/widgetbundle.js:3545
- **Description:** exec()
- **Evidence (Line):** `}`;var Ci=U4(function(){return Function(Lt,vn+"return "+Fa).apply(void 0,rr)});if(Ci.source=Fa,Tg(Ci`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 877. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/package_data/widgetbundle.js:3545
- **Description:** exec()
- **Evidence (Line):** `}`;var Ci=U4(function(){return Function(Lt,vn+"return "+Fa).apply(void 0,rr)});if(Ci.source=Fa,Tg(Ci`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 878. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pluggy/_hooks.py:512
- **Description:** exec()
- **Evidence (Line):** `return self._hookexec(self.name, self._hookimpls.copy(), kwargs, firstresult)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 879. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pluggy/_hooks.py:534
- **Description:** exec()
- **Evidence (Line):** `res = self._hookexec(self.name, self._hookimpls.copy(), kwargs, False)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 880. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pluggy/_hooks.py:573
- **Description:** exec()
- **Evidence (Line):** `return self._hookexec(self.name, hookimpls, kwargs, firstresult)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 881. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pluggy/_hooks.py:580
- **Description:** exec()
- **Evidence (Line):** `res = self._hookexec(self.name, [method], kwargs, False)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 882. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pluggy/_manager.py:111
- **Description:** exec()
- **Evidence (Line):** `def _hookexec(`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 883. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pluggy/_manager.py:120
- **Description:** exec()
- **Evidence (Line):** `return self._inner_hookexec(hook_name, methods, kwargs, firstresult)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 884. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pluggy/_manager.py:464
- **Description:** exec()
- **Evidence (Line):** `def traced_hookexec(`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 885. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pyarrow/tests/test_flight.py:2351
- **Description:** eval()
- **Evidence (Line):** `client_headers = ast.literal_eval(raw_headers)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 886. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pygments/formatters/__init__.py:91
- **Description:** eval()
- **Evidence (Line):** `this method is equivalent to running ``eval()`` on the input file. The formatter is`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 887. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pygments/formatters/__init__.py:103
- **Description:** exec()
- **Evidence (Line):** `exec(f.read(), custom_namespace)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 888. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pygments/lexers/__init__.py:154
- **Description:** exec()
- **Evidence (Line):** `exec(f.read(), custom_namespace)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 889. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pygments/lexers/_julia_builtins.py:150
- **Description:** eval()
- **Evidence (Line):** `v = eval(Symbol(compl.mod))`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 890. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pygments/lexers/_julia_builtins.py:361
- **Description:** eval()
- **Evidence (Line):** `v = eval(Symbol(compl.mod))`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 891. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pygments/lexers/special.py:115
- **Description:** eval()
- **Evidence (Line):** `val = ast.literal_eval(val)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 892. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/pytz/__init__.py:301
- **Description:** pickle load/dumps
- **Evidence (Line):** `>>> new = pickle.loads(p)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 893. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/pytz/__init__.py:507
- **Description:** pickle load/dumps
- **Evidence (Line):** `>>> pickle.loads(pickle.dumps(one)) is one`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 894. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/pytz/__init__.py:509
- **Description:** pickle load/dumps
- **Evidence (Line):** `>>> pickle.loads(pickle.dumps(two)) is two`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 895. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/_lib/_bunch.py:160
- **Description:** exec()
- **Evidence (Line):** `exec(s, namespace)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 896. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/_lib/_elementwise_iterative_method.py:227
- **Description:** eval()
- **Evidence (Line):** `x = pre_func_eval(work)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 897. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/_lib/_elementwise_iterative_method.py:250
- **Description:** eval()
- **Evidence (Line):** `post_func_eval(x, f, work)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 898. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/_lib/array_api_compat/_internal.py:64
- **Description:** exec()
- **Evidence (Line):** `exec(f"from {mod.__name__} import *", objs)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 899. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/_lib/array_api_compat/cupy/fft.py:7
- **Description:** exec()
- **Evidence (Line):** `exec("from cupy.fft import *", _n)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 900. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/_lib/array_api_compat/cupy/linalg.py:6
- **Description:** exec()
- **Evidence (Line):** `exec('from cupy.linalg import *', _n)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 901. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/_lib/cobyqa/main.py:650
- **Description:** eval()
- **Evidence (Line):** `fun_val, cub_val, ceq_val = _eval(`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 902. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/_lib/cobyqa/main.py:697
- **Description:** eval()
- **Evidence (Line):** `fun_val, cub_val, ceq_val = _eval(`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 903. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/_lib/cobyqa/main.py:840
- **Description:** eval()
- **Evidence (Line):** `fun_val, cub_val, ceq_val = _eval(pb, framework, step, options)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 904. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/_lib/cobyqa/main.py:1422
- **Description:** eval()
- **Evidence (Line):** `def _eval(pb, framework, step, options):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 905. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/_lib/cobyqa/main.py:1446
- **Description:** eval()
- **Evidence (Line):** `x, fun, maxcv = pb.best_eval(penalty)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 906. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/_lib/cobyqa/problem.py:80
- **Description:** eval()
- **Evidence (Line):** `def n_eval(self):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 907. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/_lib/cobyqa/problem.py:560
- **Description:** eval()
- **Evidence (Line):** `def n_eval(self):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 908. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/_lib/cobyqa/problem.py:886
- **Description:** eval()
- **Evidence (Line):** `x_best, fun_best, _ = self.best_eval(penalty)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 909. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/_lib/cobyqa/problem.py:947
- **Description:** eval()
- **Evidence (Line):** `def n_eval(self):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 910. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/_lib/cobyqa/problem.py:1192
- **Description:** eval()
- **Evidence (Line):** `def best_eval(self, penalty):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 911. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/_lib/tests/test_bunch.py:59
- **Description:** pickle load/dumps
- **Evidence (Line):** `obj = pickle.loads(s)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 912. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/datasets/_fetchers.py:83
- **Description:** pickle load/dumps
- **Evidence (Line):** `ascent = array(pickle.load(f))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 913. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/differentiate/_differentiate.py:447
- **Description:** eval()
- **Evidence (Line):** `def pre_func_eval(work):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 914. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/differentiate/_differentiate.py:493
- **Description:** eval()
- **Evidence (Line):** `def post_func_eval(x, f, work):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 915. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/integrate/_ivp/tests/test_ivp.py:685
- **Description:** eval()
- **Evidence (Line):** `def test_t_eval():`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 916. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/integrate/_tanhsinh.py:404
- **Description:** eval()
- **Evidence (Line):** `def pre_func_eval(work):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 917. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/integrate/_tanhsinh.py:418
- **Description:** eval()
- **Evidence (Line):** `def post_func_eval(x, fj, work):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 918. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/integrate/tests/test__quad_vec.py:146
- **Description:** eval()
- **Evidence (Line):** `def test_num_eval(self, quadrature):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 919. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/interpolate/tests/test_bsplines.py:148
- **Description:** eval()
- **Evidence (Line):** `def test_rndm_naive_eval(self, xp):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 920. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/interpolate/tests/test_bsplines.py:155
- **Description:** eval()
- **Evidence (Line):** `y_n = xp.stack([_naive_eval(x, t, c, k, xp=xp) for x in xx])`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 921. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/interpolate/tests/test_bsplines.py:915
- **Description:** eval()
- **Evidence (Line):** `def _naive_eval(x, t, c, k, *, xp):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 922. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/interpolate/tests/test_bsplines.py:1391
- **Description:** eval()
- **Evidence (Line):** `b1 = np.vectorize(lambda x: _naive_eval(x, t, c, k, xp=np))`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 923. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/interpolate/tests/test_interpnd.py:172
- **Description:** pickle load/dumps
- **Evidence (Line):** `ip2 = pickle.loads(pickle.dumps(ip))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 924. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/interpolate/tests/test_interpnd.py:413
- **Description:** pickle load/dumps
- **Evidence (Line):** `ip2 = pickle.loads(pickle.dumps(ip))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 925. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/interpolate/tests/test_interpolate.py:842
- **Description:** eval()
- **Evidence (Line):** `def test_eval(self, xp):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 926. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/interpolate/tests/test_interpolate.py:1432
- **Description:** eval()
- **Evidence (Line):** `def test_derivative_eval(self):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 927. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/interpolate/tests/test_interpolate.py:2325
- **Description:** eval()
- **Evidence (Line):** `v2 = _ppoly2d_eval(c, (x, y), xi, yi)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 928. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/interpolate/tests/test_interpolate.py:2331
- **Description:** eval()
- **Evidence (Line):** `v2 = _ppoly2d_eval(c, (x, y), xi, yi, nu=nu)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 929. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/interpolate/tests/test_interpolate.py:2351
- **Description:** eval()
- **Evidence (Line):** `v2 = _ppoly3d_eval(c, (x, y, z), xi, yi, zi, nu=nu)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 930. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/interpolate/tests/test_interpolate.py:2371
- **Description:** eval()
- **Evidence (Line):** `v2 = _ppoly4d_eval(c, (x, y, z, u), xi, yi, zi, ui)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 931. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/interpolate/tests/test_interpolate.py:2574
- **Description:** eval()
- **Evidence (Line):** `def _ppoly2d_eval(c, xs, xnew, ynew, nu=None):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 932. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/interpolate/tests/test_interpolate.py:2610
- **Description:** eval()
- **Evidence (Line):** `def _ppoly3d_eval(c, xs, xnew, ynew, znew, nu=None):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 933. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/interpolate/tests/test_interpolate.py:2650
- **Description:** eval()
- **Evidence (Line):** `def _ppoly4d_eval(c, xs, xnew, ynew, znew, unew, nu=None):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 934. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/interpolate/tests/test_polyint.py:788
- **Description:** eval()
- **Evidence (Line):** `def test_periodic_eval(self, xp):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 935. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/interpolate/tests/test_rbfinterp.py:459
- **Description:** pickle load/dumps
- **Evidence (Line):** `yitp2 = pickle.loads(pickle.dumps(interp))(xitp)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 936. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/odr/tests/test_odr.py:575
- **Description:** pickle load/dumps
- **Evidence (Line):** `pickle.loads(obj_pickle)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 937. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/odr/tests/test_odr.py:584
- **Description:** pickle load/dumps
- **Evidence (Line):** `pickle.loads(obj_pickle)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 938. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/odr/tests/test_odr.py:588
- **Description:** pickle load/dumps
- **Evidence (Line):** `pickle.loads(obj_pickle)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 939. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/odr/tests/test_odr.py:597
- **Description:** pickle load/dumps
- **Evidence (Line):** `pickle.loads(obj_pickle)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 940. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/odr/tests/test_odr.py:606
- **Description:** pickle load/dumps
- **Evidence (Line):** `pickle.loads(obj_pickle)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 941. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/optimize/_bracket.py:242
- **Description:** eval()
- **Evidence (Line):** `def pre_func_eval(work):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 942. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/optimize/_bracket.py:260
- **Description:** eval()
- **Evidence (Line):** `def post_func_eval(x, f, work):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 943. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/optimize/_bracket.py:648
- **Description:** eval()
- **Evidence (Line):** `def pre_func_eval(work):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 944. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/optimize/_bracket.py:664
- **Description:** eval()
- **Evidence (Line):** `def post_func_eval(x, f, work):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 945. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/optimize/_chandrupatla.py:154
- **Description:** eval()
- **Evidence (Line):** `def pre_func_eval(work):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 946. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/optimize/_chandrupatla.py:159
- **Description:** eval()
- **Evidence (Line):** `def post_func_eval(x, f, work):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 947. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/optimize/_chandrupatla.py:427
- **Description:** eval()
- **Evidence (Line):** `def pre_func_eval(work):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 948. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/optimize/_chandrupatla.py:464
- **Description:** eval()
- **Evidence (Line):** `def post_func_eval(x, f, work):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 949. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/optimize/_nonlin.py:1635
- **Description:** exec()
- **Evidence (Line):** `exec(wrapper, ns)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 950. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/optimize/_optimize.py:323
- **Description:** eval()
- **Evidence (Line):** `def eval(x):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 951. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/optimize/tests/test__differential_evolution.py:261
- **Description:** eval()
- **Evidence (Line):** `def test_best_solution_retrieval(self):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 952. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/optimize/tests/test_constraints.py:183
- **Description:** eval()
- **Evidence (Line):** `bounds2 = eval(repr(Bounds(*args)))`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 953. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/signal/_spline_filters.py:342
- **Description:** eval()
- **Evidence (Line):** `>>> filtered = cspline1d_eval(cspline1d(sig), time)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 954. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/signal/_spline_filters.py:396
- **Description:** eval()
- **Evidence (Line):** `>>> filtered = qspline1d_eval(qspline1d(sig), time)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 955. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/signal/_spline_filters.py:526
- **Description:** eval()
- **Evidence (Line):** `def cspline1d_eval(cj, newx, dx=1.0, x0=0):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 956. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/signal/_spline_filters.py:569
- **Description:** eval()
- **Evidence (Line):** `>>> filtered = cspline1d_eval(cspline1d(sig), time)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 957. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/signal/_spline_filters.py:592
- **Description:** eval()
- **Evidence (Line):** `res[cond1] = cspline1d_eval(cj, -newx[cond1])`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 958. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/signal/_spline_filters.py:593
- **Description:** eval()
- **Evidence (Line):** `res[cond2] = cspline1d_eval(cj, 2 * (N - 1) - newx[cond2])`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 959. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/signal/_spline_filters.py:607
- **Description:** eval()
- **Evidence (Line):** `def qspline1d_eval(cj, newx, dx=1.0, x0=0):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 960. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/signal/_spline_filters.py:652
- **Description:** eval()
- **Evidence (Line):** `>>> filtered = qspline1d_eval(qspline1d(sig), time)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 961. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/signal/_spline_filters.py:675
- **Description:** eval()
- **Evidence (Line):** `res[cond1] = qspline1d_eval(cj, -newx[cond1])`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 962. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/signal/_spline_filters.py:676
- **Description:** eval()
- **Evidence (Line):** `res[cond2] = qspline1d_eval(cj, 2 * (N - 1) - newx[cond2])`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 963. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/signal/tests/test_bsplines.py:163
- **Description:** eval()
- **Evidence (Line):** `def test_cspline1d_eval(self, xp):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 964. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/signal/tests/test_bsplines.py:164
- **Description:** eval()
- **Evidence (Line):** `r = signal.cspline1d_eval(xp.asarray([0., 0], dtype=xp.float64),`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 965. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/signal/tests/test_bsplines.py:168
- **Description:** eval()
- **Evidence (Line):** `r = signal.cspline1d_eval(xp.asarray([1., 0, 1], dtype=xp.float64),`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 966. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/signal/tests/test_bsplines.py:173
- **Description:** eval()
- **Evidence (Line):** `r = signal.cspline1d_eval(xp.asarray([1.0, 0, 1], dtype=xp.float64),`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 967. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/signal/tests/test_bsplines.py:194
- **Description:** eval()
- **Evidence (Line):** `signal.cspline1d_eval(cj, xp.asarray(newx), dx=dx, x0=x[0]), newy`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 968. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/signal/tests/test_bsplines.py:199
- **Description:** eval()
- **Evidence (Line):** `signal.cspline1d_eval(xp.asarray([], dtype=xp.float64),`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 969. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/signal/tests/test_bsplines.py:204
- **Description:** eval()
- **Evidence (Line):** `def test_qspline1d_eval(self, xp):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 970. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/signal/tests/test_bsplines.py:205
- **Description:** eval()
- **Evidence (Line):** `xp_assert_close(signal.qspline1d_eval(xp.asarray([0., 0]), xp.asarray([0.])),`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 971. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/signal/tests/test_bsplines.py:208
- **Description:** eval()
- **Evidence (Line):** `xp_assert_equal(signal.qspline1d_eval(xp.asarray([1., 0, 1]), xp.asarray([])),`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 972. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/signal/tests/test_bsplines.py:213
- **Description:** eval()
- **Evidence (Line):** `r = signal.qspline1d_eval(xp.asarray([1.0, 0, 1], dtype=xp.float64),`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 973. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/signal/tests/test_bsplines.py:233
- **Description:** eval()
- **Evidence (Line):** `r = signal.qspline1d_eval(`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 974. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/signal/tests/test_bsplines.py:240
- **Description:** eval()
- **Evidence (Line):** `signal.qspline1d_eval(xp.asarray([], dtype=xp.float64),`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 975. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/signal/tests/test_signaltools.py:1693
- **Description:** eval()
- **Evidence (Line):** `y2 = signal.cspline1d_eval(cj, x2, dx=dx, x0=x[0])`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 976. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/signal/tests/test_signaltools.py:1711
- **Description:** eval()
- **Evidence (Line):** `ynew = signal.cspline1d_eval(cy, xnew)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 977. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/sparse/csgraph/tests/test_graph_laplacian.py:44
- **Description:** eval()
- **Evidence (Line):** `mat = eval(mat, dict(np=np, sparse=sparse))`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 978. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/sparse/csgraph/tests/test_graph_laplacian.py:300
- **Description:** eval()
- **Evidence (Line):** `d[L] = eval(L)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 979. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/sparse/csgraph/tests/test_graph_laplacian.py:302
- **Description:** eval()
- **Evidence (Line):** `d[L] = eval(L)(np.eye(n, dtype=mat.dtype))`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 980. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/sparse/linalg/tests/test_interface.py:397
- **Description:** pickle load/dumps
- **Evidence (Line):** `B = pickle.loads(s)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 981. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/sparse/tests/test_base.py:2271
- **Description:** pickle load/dumps
- **Evidence (Line):** `sploaded = pickle.loads(pickle.dumps(datsp, protocol=protocol))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 982. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/spatial/tests/test_distance.py:425
- **Description:** eval()
- **Evidence (Line):** `cdist(X1, X2, metric=eval(metric), **kwargs)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 983. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/spatial/tests/test_distance.py:431
- **Description:** eval()
- **Evidence (Line):** `cdist(X1, X2, metric=eval(metric), *args)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 984. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/spatial/tests/test_distance.py:538
- **Description:** eval()
- **Evidence (Line):** `y2 = cdist(X1, X2, metric=eval(metric), **kwargs)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 985. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/spatial/tests/test_distance.py:548
- **Description:** eval()
- **Evidence (Line):** `cdist(X1, X2, metric=eval(metric), **kwargs)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 986. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/spatial/tests/test_distance.py:717
- **Description:** eval()
- **Evidence (Line):** `pdist(X1, metric=eval(metric), **kwargs)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 987. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/spatial/tests/test_distance.py:723
- **Description:** eval()
- **Evidence (Line):** `pdist(X1, metric=eval(metric), *args)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 988. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/spatial/tests/test_distance.py:1355
- **Description:** eval()
- **Evidence (Line):** `y2 = pdist(X, metric=eval(metric), **kwargs)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 989. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/spatial/tests/test_distance.py:1365
- **Description:** eval()
- **Evidence (Line):** `pdist(X, metric=eval(metric), **kwargs)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 990. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/spatial/tests/test_distance.py:2039
- **Description:** eval()
- **Evidence (Line):** `for m in [metric, eval(metric), "test_" + metric]:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 991. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/spatial/tests/test_kdtree.py:855
- **Description:** pickle load/dumps
- **Evidence (Line):** `T2 = pickle.loads(tmp)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 992. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/spatial/tests/test_kdtree.py:869
- **Description:** pickle load/dumps
- **Evidence (Line):** `T2 = pickle.loads(tmp)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 993. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/spatial/transform/tests/test_rigid_transform.py:1461
- **Description:** pickle load/dumps
- **Evidence (Line):** `unpickled = pickle.loads(pkl)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 994. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/spatial/transform/tests/test_rotation.py:2569
- **Description:** pickle load/dumps
- **Evidence (Line):** `unpickled = pickle.loads(pkl)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 995. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/special/_precompute/wright_bessel.py:183
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, n, rho, v):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 996. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/special/_precompute/wright_bessel.py:203
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, m, rho, beta):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 997. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/special/tests/test_orthogonal.py:282
- **Description:** eval()
- **Evidence (Line):** `p = eval(pstr)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 998. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/special/tests/test_support_alternative_backends.py:332
- **Description:** pickle load/dumps
- **Evidence (Line):** `roundtrip = pickle.loads(pickle.dumps(func))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 999. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/stats/_continued_fraction.py:325
- **Description:** eval()
- **Evidence (Line):** `def pre_func_eval(work):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1000. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/stats/_continued_fraction.py:329
- **Description:** eval()
- **Evidence (Line):** `def post_func_eval(n, ab, work):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1001. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/stats/_continuous_distns.py:3460
- **Description:** eval()
- **Evidence (Line):** `def sk1_eval(c, *args):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1002. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/stats/_continuous_distns.py:3472
- **Description:** eval()
- **Evidence (Line):** `def ku1_eval(c, *args):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1003. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/stats/_distn_infrastructure.py:368
- **Description:** exec()
- **Evidence (Line):** `exec('del ' + obj)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1004. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/stats/_distn_infrastructure.py:747
- **Description:** exec()
- **Evidence (Line):** `exec(self._parse_arg_template, ns)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1005. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/stats/tests/common_tests.py:296
- **Description:** pickle load/dumps
- **Evidence (Line):** `unpickled = pickle.loads(s)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1006. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/stats/tests/common_tests.py:309
- **Description:** pickle load/dumps
- **Evidence (Line):** `unpickled = pickle.loads(pkl)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1007. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/stats/tests/common_tests.py:319
- **Description:** pickle load/dumps
- **Evidence (Line):** `unpickled_fit_function = pickle.loads(pickled_fit_function)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1008. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/stats/tests/test_axis_nan_policy.py:816
- **Description:** pickle load/dumps
- **Evidence (Line):** `unpickled_hypotest = pickle.loads(pickled_hypotest)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1009. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/stats/tests/test_continued_fraction.py:149
- **Description:** eval()
- **Evidence (Line):** `def test_feval(self, xp):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1010. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/stats/tests/test_continuous.py:87
- **Description:** eval()
- **Evidence (Line):** `ref = eval(f'(a {left_comparison} x) & (x {right_comparison} b)')`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1011. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/stats/tests/test_continuous.py:2034
- **Description:** eval()
- **Evidence (Line):** `new_dist = eval(repr(dist))`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1012. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/stats/tests/test_continuous.py:956
- **Description:** pickle load/dumps
- **Evidence (Line):** `dist3 = pickle.loads(pickle.dumps(dist1))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1013. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/stats/tests/test_distributions.py:6883
- **Description:** pickle load/dumps
- **Evidence (Line):** `unpickled = pickle.loads(s)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1014. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/stats/tests/test_multivariate.py:4370
- **Description:** pickle load/dumps
- **Evidence (Line):** `unpickled = pickle.loads(s)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1015. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/scipy/stats/tests/test_sampling.py:260
- **Description:** pickle load/dumps
- **Evidence (Line):** `rng2 = pickle.loads(obj)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1016. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/sdmetrics/reports/base_report.py:378
- **Description:** pickle load/dumps
- **Evidence (Line):** `report = pickle.load(f)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1017. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/sdv/lite/single_table.py:203
- **Description:** pickle load/dumps
- **Evidence (Line):** `model = cloudpickle.load(f)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1018. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/sdv/multi_table/base.py:962
- **Description:** pickle load/dumps
- **Evidence (Line):** `synthesizer = cloudpickle.load(f)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1019. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/sdv/single_table/base.py:826
- **Description:** pickle load/dumps
- **Evidence (Line):** `synthesizer = cloudpickle.load(f)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1020. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/sdv/utils/utils.py:173
- **Description:** pickle load/dumps
- **Evidence (Line):** `synthesizer = cloudpickle.load(f)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1021. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/setuptools/_distutils/command/bdist_rpm.py:369
- **Description:** os.popen()
- **Evidence (Line):** `out = os.popen(q_cmd)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1022. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/setuptools/_distutils/compilers/C/base.py:1130
- **Description:** eval()
- **Evidence (Line):** `if lib_type not in eval(expected):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1023. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/setuptools/_distutils/core.py:210
- **Description:** exec()
- **Evidence (Line):** `'script_name' is a file that will be read and run with 'exec()';`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1024. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/setuptools/_distutils/core.py:250
- **Description:** exec()
- **Evidence (Line):** `exec(code, g)  # noqa: S102 # executing the setup script is the point`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1025. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/setuptools/_vendor/jaraco/functools/__init__.py:563
- **Description:** eval()
- **Evidence (Line):** `return eval(use)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1026. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/setuptools/_vendor/more_itertools/recipes.py:1155
- **Description:** eval()
- **Evidence (Line):** `def polynomial_eval(coefficients, x):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1027. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/setuptools/_vendor/more_itertools/recipes.py:1164
- **Description:** eval()
- **Evidence (Line):** `>>> polynomial_eval(coefficients, x)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1028. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/setuptools/_vendor/packaging/_parser.py:344
- **Description:** eval()
- **Evidence (Line):** `value = ast.literal_eval(python_str)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1029. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/setuptools/build_meta.py:317
- **Description:** exec()
- **Evidence (Line):** `exec(code, locals())  # noqa: S102 # exec is intentional here`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1030. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/setuptools/command/bdist_egg.py:396
- **Description:** marshal load
- **Evidence (Line):** `code = marshal.load(f)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1031. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/setuptools/config/expand.py:72
- **Description:** eval()
- **Evidence (Line):** `ast.literal_eval(value)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1032. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/setuptools/depends.py:132
- **Description:** marshal load
- **Evidence (Line):** `code = marshal.load(f)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1033. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/setuptools/launch.py:32
- **Description:** exec()
- **Evidence (Line):** `exec(code, namespace)  # noqa: S102 # exec is intentional here`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1034. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/setuptools/wheel.py:191
- **Description:** eval()
- **Evidence (Line):** `def eval(req, **env):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1035. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/setuptools/wheel.py:212
- **Description:** eval()
- **Evidence (Line):** `(req for req in reqs if for_extra(req) and eval(req, extra=extra)),`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1036. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/six.py:740
- **Description:** exec()
- **Evidence (Line):** `exec("""exec _code_ in _globs_, _locs_""")`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1037. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/sklearn/_loss/tests/test_loss.py:1329
- **Description:** pickle load/dumps
- **Evidence (Line):** `unpickled_loss = pickle.loads(pickled_loss)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1038. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/sklearn/callback/tests/test_pickle.py:46
- **Description:** pickle load/dumps
- **Evidence (Line):** `restored = pickle.loads(pickle.dumps(estimator))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1039. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/sklearn/callback/tests/test_pickle.py:57
- **Description:** pickle load/dumps
- **Evidence (Line):** `restored = pickle.loads(pickle.dumps(estimator))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1040. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/sklearn/callback/tests/test_pickle.py:83
- **Description:** pickle load/dumps
- **Evidence (Line):** `restored = pickle.loads(pickle.dumps(estimator))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1041. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/sklearn/callback/tests/test_pickle.py:127
- **Description:** pickle load/dumps
- **Evidence (Line):** `est = pickle.load(f)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1042. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sklearn/callback/tests/test_scoring_monitor.py:325
- **Description:** eval()
- **Evidence (Line):** `def test_get_logs_include_lineage_ancestor_retrieval(as_pandas):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1043. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/sklearn/cluster/tests/test_dbscan.py:299
- **Description:** pickle load/dumps
- **Evidence (Line):** `assert type(pickle.loads(s)) is obj.__class__`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1044. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/sklearn/cluster/tests/test_spectral.py:72
- **Description:** pickle load/dumps
- **Evidence (Line):** `model_copy = pickle.loads(pickle.dumps(model))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1045. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/sklearn/compose/tests/test_column_transformer.py:1464
- **Description:** pickle load/dumps
- **Evidence (Line):** `selector_picked = pickle.loads(pickle.dumps(selector))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1046. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/sklearn/datasets/_twenty_newsgroups.py:310
- **Description:** pickle load/dumps
- **Evidence (Line):** `cache = pickle.loads(uncompressed_content)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1047. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/sklearn/ensemble/_hist_gradient_boosting/tests/test_gradient_boosting.py:1719
- **Description:** pickle load/dumps
- **Evidence (Line):** `new_clf = pickle.load(pickle_dump_with_different_bitness())`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1048. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/sklearn/ensemble/tests/test_forest.py:721
- **Description:** pickle load/dumps
- **Evidence (Line):** `obj2 = pickle.loads(pickle_object)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1049. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/sklearn/ensemble/tests/test_gradient_boosting.py:485
- **Description:** pickle load/dumps
- **Evidence (Line):** `clf = pickle.loads(serialized_clf)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1050. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/sklearn/ensemble/tests/test_weight_boosting.py:189
- **Description:** pickle load/dumps
- **Evidence (Line):** `obj2 = pickle.loads(s)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1051. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/sklearn/ensemble/tests/test_weight_boosting.py:200
- **Description:** pickle load/dumps
- **Evidence (Line):** `obj2 = pickle.loads(s)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1052. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sklearn/externals/array_api_compat/_internal.py:64
- **Description:** exec()
- **Evidence (Line):** `exec(f"from {mod.__name__} import *", objs)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1053. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sklearn/externals/array_api_compat/cupy/fft.py:7
- **Description:** exec()
- **Evidence (Line):** `exec("from cupy.fft import *", _n)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1054. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sklearn/externals/array_api_compat/cupy/linalg.py:10
- **Description:** exec()
- **Evidence (Line):** `exec('from cupy.linalg import *', _n)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1055. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/sklearn/feature_extraction/tests/test_text.py:1051
- **Description:** pickle load/dumps
- **Evidence (Line):** `copy = pickle.loads(s)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1056. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/sklearn/feature_extraction/tests/test_text.py:1075
- **Description:** pickle load/dumps
- **Evidence (Line):** `roundtripped_function = pickle.loads(pickle.dumps(function))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1057. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/sklearn/feature_extraction/tests/test_text.py:1101
- **Description:** pickle load/dumps
- **Evidence (Line):** `unpickled_cv = pickle.loads(pickle.dumps(cv))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1058. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/sklearn/feature_extraction/tests/test_text.py:1130
- **Description:** pickle load/dumps
- **Evidence (Line):** `unpickled_cv = pickle.loads(pickle.dumps(cv))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1059. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/sklearn/feature_extraction/tests/test_text.py:1142
- **Description:** pickle load/dumps
- **Evidence (Line):** `copy = pickle.loads(s)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1060. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/sklearn/linear_model/tests/test_sgd.py:919
- **Description:** pickle load/dumps
- **Evidence (Line):** `clf = pickle.loads(pickle.dumps(clf))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1061. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sklearn/metrics/_ranking.py:1749
- **Description:** eval()
- **Evidence (Line):** `European conference on information retrieval (pp. 414-421). Springer,`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1062. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sklearn/metrics/_ranking.py:1855
- **Description:** eval()
- **Evidence (Line):** `European conference on information retrieval (pp. 414-421). Springer,`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1063. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sklearn/metrics/_ranking.py:2017
- **Description:** eval()
- **Evidence (Line):** `European conference on information retrieval (pp. 414-421). Springer,`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1064. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/sklearn/metrics/tests/test_dist_metrics.py:258
- **Description:** pickle load/dumps
- **Evidence (Line):** `dm2 = pickle.loads(pickle.dumps(dm))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1065. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/sklearn/metrics/tests/test_dist_metrics.py:268
- **Description:** pickle load/dumps
- **Evidence (Line):** `dm2 = pickle.loads(pickle.dumps(dm))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1066. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/sklearn/metrics/tests/test_dist_metrics.py:327
- **Description:** pickle load/dumps
- **Evidence (Line):** `euclidean_pkl = pickle.loads(pickle.dumps(euclidean))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1067. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/sklearn/metrics/tests/test_dist_metrics.py:328
- **Description:** pickle load/dumps
- **Evidence (Line):** `pyfunc_pkl = pickle.loads(pickle.dumps(pyfunc))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1068. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/sklearn/metrics/tests/test_score_objects.py:458
- **Description:** pickle load/dumps
- **Evidence (Line):** `unpickled_scorer = pickle.loads(pickle.dumps(scorer))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1069. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/sklearn/model_selection/tests/test_search.py:1618
- **Description:** pickle load/dumps
- **Evidence (Line):** `grid_search_pickled = pickle.loads(pickle.dumps(grid_search))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1070. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/sklearn/model_selection/tests/test_search.py:1625
- **Description:** pickle load/dumps
- **Evidence (Line):** `random_search_pickled = pickle.loads(pickle.dumps(random_search))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1071. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/sklearn/neighbors/tests/test_nearest_centroid.py:116
- **Description:** pickle load/dumps
- **Evidence (Line):** `obj2 = pickle.loads(s)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1072. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/sklearn/neighbors/tests/test_neighbors_tree.py:289
- **Description:** pickle load/dumps
- **Evidence (Line):** `tree2 = pickle.loads(s)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1073. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/sklearn/neighbors/tests/test_quad_tree.py:74
- **Description:** pickle load/dumps
- **Evidence (Line):** `bt2 = pickle.loads(s)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1074. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/sklearn/tests/test_base.py:507
- **Description:** pickle load/dumps
- **Evidence (Line):** `tree_restored = pickle.loads(tree_pickle)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1075. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/sklearn/tests/test_base.py:539
- **Description:** pickle load/dumps
- **Evidence (Line):** `pickle.loads(tree_pickle_other)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1076. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/sklearn/tests/test_base.py:567
- **Description:** pickle load/dumps
- **Evidence (Line):** `pickle.loads(tree_pickle_noversion)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1077. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/sklearn/tests/test_base.py:583
- **Description:** pickle load/dumps
- **Evidence (Line):** `pickle.loads(tree_pickle_noversion)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1078. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/sklearn/tests/test_base.py:610
- **Description:** pickle load/dumps
- **Evidence (Line):** `estimator_restored = pickle.loads(serialized)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1079. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/sklearn/tests/test_base.py:651
- **Description:** pickle load/dumps
- **Evidence (Line):** `estimator_restored = pickle.loads(serialized)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1080. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/sklearn/tests/test_base.py:891
- **Description:** pickle load/dumps
- **Evidence (Line):** `pickle.loads(pickle.dumps(BaseEstimator()))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1081. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/sklearn/tests/test_isotonic.py:379
- **Description:** pickle load/dumps
- **Evidence (Line):** `ir2 = pickle.loads(ir_ser)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1082. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/sklearn/tree/tests/test_tree.py:909
- **Description:** pickle load/dumps
- **Evidence (Line):** `est2 = pickle.loads(serialized_object)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1083. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/sklearn/tree/tests/test_tree.py:1849
- **Description:** pickle load/dumps
- **Evidence (Line):** `return pickle.loads(pickle.dumps(obj))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1084. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/sklearn/tree/tests/test_tree.py:2155
- **Description:** pickle load/dumps
- **Evidence (Line):** `new_clf = pickle.load(get_pickle_non_native_endianness())`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1085. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/sklearn/tree/tests/test_tree.py:2249
- **Description:** pickle load/dumps
- **Evidence (Line):** `new_clf = pickle.load(pickle_dump_with_different_bitness())`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1086. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/sklearn/tree/tests/test_tree.py:2397
- **Description:** pickle load/dumps
- **Evidence (Line):** `splitter_back = pickle.loads(splitter_serialize)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1087. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/sklearn/utils/estimator_checks.py:2717
- **Description:** pickle load/dumps
- **Evidence (Line):** `unpickled_estimator = pickle.loads(pickled_estimator)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1088. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/sklearn/utils/estimator_checks.py:3975
- **Description:** pickle load/dumps
- **Evidence (Line):** `est = pickle.loads(pickle.dumps(est))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1089. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/sklearn/utils/tests/test_deprecation.py:89
- **Description:** pickle load/dumps
- **Evidence (Line):** `pickle.loads(pickle.dumps(mock_function))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1090. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/sklearn/utils/tests/test_encode.py:140
- **Description:** pickle load/dumps
- **Evidence (Line):** `uniques = pickle.loads(pickle.dumps(uniques))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1091. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/sklearn/utils/tests/test_encode.py:149
- **Description:** pickle load/dumps
- **Evidence (Line):** `uniques = pickle.loads(pickle.dumps(uniques))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1092. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/sklearn/utils/tests/test_encode.py:159
- **Description:** pickle load/dumps
- **Evidence (Line):** `uniques = pickle.loads(pickle.dumps(uniques))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1093. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/sklearn/utils/tests/test_encode.py:182
- **Description:** pickle load/dumps
- **Evidence (Line):** `uniques = pickle.loads(pickle.dumps(uniques))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1094. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/sklearn/utils/tests/test_metaestimators.py:62
- **Description:** pickle load/dumps
- **Evidence (Line):** `unpickled_func = pickle.loads(pickled_bytes)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1095. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/dialects/mssql/base.py:1880
- **Description:** exec()
- **Evidence (Line):** `def pre_exec(self):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1096. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/dialects/mssql/base.py:1929
- **Description:** exec()
- **Evidence (Line):** `def post_exec(self):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1097. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/dialects/mssql/pyodbc.py:567
- **Description:** exec()
- **Evidence (Line):** `def pre_exec(self):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1098. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/dialects/mssql/pyodbc.py:580
- **Description:** exec()
- **Evidence (Line):** `super().pre_exec()`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1099. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/dialects/mssql/pyodbc.py:593
- **Description:** exec()
- **Evidence (Line):** `def post_exec(self):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1100. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/dialects/mssql/pyodbc.py:619
- **Description:** exec()
- **Evidence (Line):** `super().post_exec()`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1101. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/dialects/mysql/base.py:1285
- **Description:** exec()
- **Evidence (Line):** `def post_exec(self) -> None:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1102. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/dialects/mysql/mariadbconnector.py:104
- **Description:** exec()
- **Evidence (Line):** `def post_exec(self) -> None:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1103. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/dialects/mysql/mariadbconnector.py:105
- **Description:** exec()
- **Evidence (Line):** `super().post_exec()`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1104. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/dialects/oracle/base.py:1920
- **Description:** exec()
- **Evidence (Line):** `def pre_exec(self):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1105. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/dialects/oracle/cx_oracle.py:947
- **Description:** exec()
- **Evidence (Line):** `def pre_exec(self):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1106. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/dialects/oracle/cx_oracle.py:948
- **Description:** exec()
- **Evidence (Line):** `super().pre_exec()`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1107. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/dialects/oracle/cx_oracle.py:958
- **Description:** exec()
- **Evidence (Line):** `def post_exec(self):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1108. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/dialects/postgresql/asyncpg.py:458
- **Description:** exec()
- **Evidence (Line):** `def pre_exec(self):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1109. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/dialects/postgresql/pg8000.py:331
- **Description:** exec()
- **Evidence (Line):** `def pre_exec(self):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1110. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/dialects/postgresql/psycopg2.py:574
- **Description:** exec()
- **Evidence (Line):** `def post_exec(self):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1111. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/engine/base.py:1843
- **Description:** exec()
- **Evidence (Line):** `context.pre_exec()`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1112. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/engine/base.py:1983
- **Description:** exec()
- **Evidence (Line):** `context.post_exec()`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1113. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/engine/base.py:2159
- **Description:** exec()
- **Evidence (Line):** `context.post_exec()`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1114. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/engine/default.py:1788
- **Description:** exec()
- **Evidence (Line):** `def pre_exec(self):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1115. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/engine/default.py:1796
- **Description:** exec()
- **Evidence (Line):** `def post_exec(self):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1116. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/engine/default.py:1816
- **Description:** exec()
- **Evidence (Line):** `calculated within post_exec().`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1117. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/engine/interfaces.py:2996
- **Description:** exec()
- **Evidence (Line):** `sql.Compiled object by the time pre_exec() has completed."""`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1118. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/engine/interfaces.py:3140
- **Description:** exec()
- **Evidence (Line):** `def pre_exec(self) -> None:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1119. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/engine/interfaces.py:3186
- **Description:** exec()
- **Evidence (Line):** `def post_exec(self) -> None:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1120. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/ext/serializer.py:64
- **Description:** pickle load/dumps
- **Evidence (Line):** `pickle.loads()/dumps() can be used to fully dump any ``MetaData`` object,`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1121. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/ext/serializer.py:149
- **Description:** pickle load/dumps
- **Evidence (Line):** `cls = pickle.loads(b64decode(clsarg))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1122. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/ext/serializer.py:152
- **Description:** pickle load/dumps
- **Evidence (Line):** `cls = pickle.loads(b64decode(args))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1123. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/ext/serializer.py:155
- **Description:** pickle load/dumps
- **Evidence (Line):** `cls = pickle.loads(b64decode(args))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1124. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/ext/serializer.py:159
- **Description:** pickle load/dumps
- **Evidence (Line):** `cls = pickle.loads(b64decode(mapper))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1125. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/orm/_orm_constructors.py:1142
- **Description:** eval()
- **Evidence (Line):** `argument is interpreted using Python's ``eval()`` function.`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1126. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/orm/_orm_constructors.py:1379
- **Description:** eval()
- **Evidence (Line):** `argument is interpreted using Python's ``eval()`` function.`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1127. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/orm/_orm_constructors.py:1581
- **Description:** eval()
- **Evidence (Line):** `argument is interpreted using Python's ``eval()`` function.`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1128. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/orm/_orm_constructors.py:1678
- **Description:** eval()
- **Evidence (Line):** `argument is interpreted using Python's ``eval()`` function.`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1129. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/orm/_orm_constructors.py:1697
- **Description:** eval()
- **Evidence (Line):** `argument is interpreted using Python's ``eval()`` function.`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1130. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/orm/_orm_constructors.py:1739
- **Description:** eval()
- **Evidence (Line):** `argument is interpreted using Python's ``eval()`` function.`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1131. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/orm/bulk_persistence.py:675
- **Description:** exec()
- **Evidence (Line):** `def orm_pre_session_exec(`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1132. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/orm/bulk_persistence.py:1193
- **Description:** exec()
- **Evidence (Line):** `def orm_pre_session_exec(`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1133. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/orm/clsregistry.py:533
- **Description:** eval()
- **Evidence (Line):** `x = eval(self.arg, globals(), self._dict)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1134. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/orm/context.py:285
- **Description:** exec()
- **Evidence (Line):** `def orm_pre_session_exec(`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1135. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/orm/context.py:335
- **Description:** exec()
- **Evidence (Line):** `def orm_pre_session_exec(`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1136. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/orm/context.py:492
- **Description:** exec()
- **Evidence (Line):** `def orm_pre_session_exec(`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1137. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/orm/events.py:1997
- **Description:** exec()
- **Evidence (Line):** `def after_flush_postexec(`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1138. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/orm/instrumentation.py:743
- **Description:** exec()
- **Evidence (Line):** `exec(func_text, env)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1139. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/orm/session.py:2205
- **Description:** exec()
- **Evidence (Line):** `) = compile_state_cls.orm_pre_session_exec(`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1140. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/orm/session.py:2242
- **Description:** exec()
- **Evidence (Line):** `# now run orm_pre_session_exec() "for real".   if there were`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1141. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/orm/session.py:2250
- **Description:** exec()
- **Evidence (Line):** `) = compile_state_cls.orm_pre_session_exec(`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1142. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/orm/session.py:4483
- **Description:** exec()
- **Evidence (Line):** `self.dispatch.after_flush_postexec(self, flush_context)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1143. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/orm/strategies.py:2892
- **Description:** exec()
- **Evidence (Line):** `def load_collection_from_joined_exec(state, dict_, row):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1144. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/orm/strategies.py:2931
- **Description:** exec()
- **Evidence (Line):** `def load_scalar_from_joined_exec(state, dict_, row):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1145. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/sql/lambdas.py:1263
- **Description:** exec()
- **Evidence (Line):** `exec(code, vars_, vars_)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1146. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/sql/sqltypes.py:1854
- **Description:** pickle load/dumps
- **Evidence (Line):** ```pickle.dumps()`` to incoming objects, and ``pickle.loads()`` on`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1147. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/testing/plugin/pytestplugin.py:646
- **Description:** exec()
- **Evidence (Line):** `exec(code, env)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1148. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/util/langhelpers.py:311
- **Description:** exec()
- **Evidence (Line):** `exec(code, env)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1149. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/util/langhelpers.py:949
- **Description:** exec()
- **Evidence (Line):** `exec(py, env)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1150. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/util/langhelpers.py:1947
- **Description:** exec()
- **Evidence (Line):** `exec(code, env)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1151. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/util/typing.py:274
- **Description:** eval()
- **Evidence (Line):** `annotation = eval(expression, cls_namespace, locals_)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1152. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/util/typing.py:276
- **Description:** eval()
- **Evidence (Line):** `annotation = eval(expression, base_globals, locals_)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1153. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/util/typing.py:302
- **Description:** eval()
- **Evidence (Line):** `# name only, just look in globals.  eval() works perfectly fine here,`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1154. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/__init__.py:47
- **Description:** eval()
- **Evidence (Line):** `return eval(debug_str)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1155. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/abc.py:96
- **Description:** exec()
- **Evidence (Line):** `exec('from sympy import *', ns)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1156. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/assumptions/assume.py:156
- **Description:** eval()
- **Evidence (Line):** `return self.function.eval(self.arguments, assumptions)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1157. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/assumptions/assume.py:340
- **Description:** eval()
- **Evidence (Line):** `def eval(self, args, assumptions=True):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1158. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/assumptions/assume.py:426
- **Description:** eval()
- **Evidence (Line):** `def eval(self, args, assumptions=True):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1159. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/assumptions/relation/binrel.py:113
- **Description:** eval()
- **Evidence (Line):** `def eval(self, args, assumptions=True):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1160. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/assumptions/relation/binrel.py:200
- **Description:** eval()
- **Evidence (Line):** `ret = self.function.eval(self.arguments, assumptions)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1161. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/assumptions/relation/binrel.py:206
- **Description:** eval()
- **Evidence (Line):** `return self.function.eval(args, assumptions)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1162. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/assumptions/relation/equality.py:62
- **Description:** eval()
- **Evidence (Line):** `def eval(self, args, assumptions=True):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1163. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/assumptions/relation/equality.py:106
- **Description:** eval()
- **Evidence (Line):** `def eval(self, args, assumptions=True):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1164. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/assumptions/relation/equality.py:154
- **Description:** eval()
- **Evidence (Line):** `def eval(self, args, assumptions=True):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1165. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/assumptions/relation/equality.py:202
- **Description:** eval()
- **Evidence (Line):** `def eval(self, args, assumptions=True):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1166. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/assumptions/relation/equality.py:250
- **Description:** eval()
- **Evidence (Line):** `def eval(self, args, assumptions=True):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1167. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/assumptions/relation/equality.py:298
- **Description:** eval()
- **Evidence (Line):** `def eval(self, args, assumptions=True):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1168. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/assumptions/wrapper.py:90
- **Description:** eval()
- **Evidence (Line):** `...     def eval(cls, x, assumptions=True):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1169. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/assumptions/wrapper.py:97
- **Description:** eval()
- **Evidence (Line):** `...         return MyAbs.eval(self.args[0], assumptions)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1170. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/benchmarks/bench_meijerint.py:249
- **Description:** exec()
- **Evidence (Line):** `exec(string)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1171. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/codegen/algorithms.py:154
- **Description:** eval()
- **Evidence (Line):** `>>> res = eval('newton(0.5)', namespace)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1172. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/codegen/algorithms.py:153
- **Description:** exec()
- **Evidence (Line):** `>>> exec(py_mod, namespace, namespace)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1173. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/codegen/ast.py:105
- **Description:** exec()
- **Evidence (Line):** `>>> exec(py_str)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1174. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/codegen/cfunctions.py:70
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, arg):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1175. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/codegen/cfunctions.py:71
- **Description:** eval()
- **Evidence (Line):** `exp_arg = exp.eval(arg)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1176. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/codegen/cfunctions.py:138
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, arg):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1177. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/codegen/cfunctions.py:142
- **Description:** eval()
- **Evidence (Line):** `return log.eval(arg + S.One)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1178. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/codegen/cfunctions.py:215
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, arg):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1179. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/codegen/cfunctions.py:264
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, arg):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1180. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/codegen/cfunctions.py:266
- **Description:** eval()
- **Evidence (Line):** `result = log.eval(arg, base=_Two)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1181. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/codegen/cfunctions.py:368
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, arg):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1182. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/codegen/cfunctions.py:370
- **Description:** eval()
- **Evidence (Line):** `result = log.eval(arg, base=_Ten)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1183. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/codegen/cfunctions.py:539
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, arg):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1184. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/codegen/cfunctions.py:552
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, arg):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1185. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/codegen/tests/test_algorithms.py:82
- **Description:** eval()
- **Evidence (Line):** `res = eval('newton(0.5)', namespace)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1186. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/codegen/tests/test_algorithms.py:81
- **Description:** exec()
- **Evidence (Line):** `exec(py_mod, namespace, namespace)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1187. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/codegen/tests/test_algorithms.py:170
- **Description:** exec()
- **Evidence (Line):** `exec(v, ns, ns)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1188. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/combinatorics/schur_number.py:40
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, k):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1189. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/core/function.py:309
- **Description:** eval()
- **Evidence (Line):** `evaluated = cls.eval(*args)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1190. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/core/function.py:340
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, *args):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1191. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/core/function.py:347
- **Description:** eval()
- **Evidence (Line):** `The ``eval()`` method is called when the class ``cls`` is about to be`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1192. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/core/function.py:352
- **Description:** eval()
- **Evidence (Line):** `Examples of ``eval()`` for the function "sign"`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1193. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/core/function.py:357
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, arg):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1194. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/core/function.py:1653
- **Description:** eval()
- **Evidence (Line):** `def eval(x):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1195. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/core/mod.py:56
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, p, q):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1196. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/core/mod.py:57
- **Description:** eval()
- **Evidence (Line):** `def number_eval(p, q):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1197. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/core/mod.py:115
- **Description:** eval()
- **Evidence (Line):** `rv = number_eval(p, q)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1198. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/core/mod.py:224
- **Description:** eval()
- **Evidence (Line):** `rv = number_eval(p, q)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1199. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/core/symbol.py:479
- **Description:** eval()
- **Evidence (Line):** `# >>> d2 = eval(srepr(d1))`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1200. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/core/sympify.py:209
- **Description:** exec()
- **Evidence (Line):** `>>> exec('from sympy.core.evalf import bitcount', ns)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1201. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/core/sympify.py:219
- **Description:** exec()
- **Evidence (Line):** `>>> exec('from sympy.abc import O', ns)  # method 2`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1202. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/core/tests/test_basic.py:325
- **Description:** exec()
- **Evidence (Line):** `exec(code)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1203. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/core/tests/test_eval.py:11
- **Description:** eval()
- **Evidence (Line):** `def test_add_eval():`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1204. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/core/tests/test_eval.py:25
- **Description:** eval()
- **Evidence (Line):** `def test_addmul_eval():`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1205. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/core/tests/test_eval.py:36
- **Description:** eval()
- **Evidence (Line):** `def test_pow_eval():`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1206. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/core/tests/test_eval.py:65
- **Description:** eval()
- **Evidence (Line):** `def test_mulpow_eval():`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1207. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/core/tests/test_function.py:140
- **Description:** eval()
- **Evidence (Line):** `def eval(cls):  # zero args`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1208. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/core/tests/test_function.py:149
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, x):  # one arg`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1209. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/core/tests/test_function.py:158
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, *x):  # star args`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1210. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/core/tests/test_function.py:518
- **Description:** eval()
- **Evidence (Line):** `def test_extensibility_eval():`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1211. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/core/tests/test_function.py:521
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, *args):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1212. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/core/tests/test_function.py:1258
- **Description:** eval()
- **Evidence (Line):** `def test_undefined_function_eval():`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1213. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/core/tests/test_function.py:1431
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, x):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1214. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/core/tests/test_function.py:1440
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, x):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1215. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/core/tests/test_function.py:1453
- **Description:** eval()
- **Evidence (Line):** `def eval(self, x):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1216. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/core/tests/test_numbers.py:678
- **Description:** eval()
- **Evidence (Line):** `def test_Float_eval():`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1217. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/core/tests/test_singleton.py:57
- **Description:** exec()
- **Evidence (Line):** `exec('from sympy import *', d)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1218. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/core/tests/test_subs.py:107
- **Description:** eval()
- **Evidence (Line):** `def test_logexppow():   # no eval()`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1219. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/core/tests/test_sympify.py:549
- **Description:** exec()
- **Evidence (Line):** `exec("from sympy.abc import Q, C", locals)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1220. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/core/tests/test_var.py:7
- **Description:** eval()
- **Evidence (Line):** `eval("var('a')", ns)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1221. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/core/tests/test_var.py:10
- **Description:** eval()
- **Evidence (Line):** `eval("var('b bb cc zz _x')", ns)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1222. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/core/tests/test_var.py:17
- **Description:** eval()
- **Evidence (Line):** `v = eval("var(['d', 'e', 'fg'])", ns)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1223. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/core/tests/test_var.py:30
- **Description:** eval()
- **Evidence (Line):** `v2 = eval("var('q')", ns)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1224. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/core/tests/test_var.py:31
- **Description:** eval()
- **Evidence (Line):** `v3 = eval("var('q p')", ns)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1225. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/core/tests/test_var.py:39
- **Description:** eval()
- **Evidence (Line):** `v1 = eval("var('x y z')", ns)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1226. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/core/tests/test_var.py:40
- **Description:** eval()
- **Evidence (Line):** `v2 = eval("var('x,y,z')", ns)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1227. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/core/tests/test_var.py:41
- **Description:** eval()
- **Evidence (Line):** `v3 = eval("var('x,y z')", ns)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1228. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/core/tests/test_var.py:49
- **Description:** eval()
- **Evidence (Line):** `eval("var('x y', real=True)", ns)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1229. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/core/tests/test_var.py:55
- **Description:** eval()
- **Evidence (Line):** `eval("var('f', cls=Function)", ns)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1230. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/core/tests/test_var.py:59
- **Description:** eval()
- **Evidence (Line):** `eval("var('g,h', cls=Function)", ns)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1231. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/external/importtools.py:21
- **Description:** eval()
- **Evidence (Line):** `return eval(debug_str)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1232. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/external/tests/test_codegen.py:122
- **Description:** subprocess with shell=True
- **Evidence (Line):** `retcode = subprocess.call(command, stdout=null, shell=True,`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1233. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/external/tests/test_pythonmpq.py:97
- **Description:** pickle load/dumps
- **Evidence (Line):** `assert pickle.loads(pickle.dumps(q)) == q`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1234. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/combinatorial/factorials.py:139
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, n):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1235. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/combinatorial/factorials.py:329
- **Description:** eval()
- **Evidence (Line):** `def _eval(self, n):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1236. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/combinatorial/factorials.py:341
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, arg):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1237. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/combinatorial/factorials.py:344
- **Description:** eval()
- **Evidence (Line):** `return cls._eval(arg)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1238. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/combinatorial/factorials.py:422
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, arg):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1239. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/combinatorial/factorials.py:566
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, x, k):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1240. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/combinatorial/factorials.py:728
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, x, k):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1241. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/combinatorial/factorials.py:938
- **Description:** eval()
- **Evidence (Line):** `def _eval(self, n, k):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1242. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/combinatorial/factorials.py:969
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, n, k):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1243. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/combinatorial/factorials.py:983
- **Description:** eval()
- **Evidence (Line):** `res = cls._eval(n, k)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1244. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/combinatorial/numbers.py:241
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, n, sym=None):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1245. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/combinatorial/numbers.py:310
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, n):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1246. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/combinatorial/numbers.py:379
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, n, sym=None):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1247. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/combinatorial/numbers.py:544
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, n, x=None):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1248. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/combinatorial/numbers.py:724
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, n, k_sym=None, symbols=None):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1249. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/combinatorial/numbers.py:898
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, n, m=None):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1250. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/combinatorial/numbers.py:1120
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, n, x=None):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1251. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/combinatorial/numbers.py:1285
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, n):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1252. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/combinatorial/numbers.py:1402
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, n, x=None):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1253. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/combinatorial/numbers.py:1549
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, n):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1254. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/combinatorial/numbers.py:1640
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, n):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1255. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/combinatorial/numbers.py:1707
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, n, k=S.One):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1256. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/combinatorial/numbers.py:1786
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, n, k=S.One):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1257. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/combinatorial/numbers.py:1834
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, a, p):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1258. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/combinatorial/numbers.py:1905
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, m, n):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1259. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/combinatorial/numbers.py:1948
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, a, n):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1260. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/combinatorial/numbers.py:2008
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, n):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1261. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/combinatorial/numbers.py:2078
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, n):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1262. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/combinatorial/numbers.py:2131
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, n):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1263. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/combinatorial/numbers.py:2179
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, n):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1264. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/combinatorial/numbers.py:2229
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, n):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1265. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/combinatorial/numbers.py:2292
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, n):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1266. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/combinatorial/numbers.py:3070
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, n):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1267. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/combinatorial/tests/test_comb_numbers.py:1183
- **Description:** eval()
- **Evidence (Line):** `raises(ValueError, lambda: motzkin.eval(77.58))`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1268. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/combinatorial/tests/test_comb_numbers.py:1184
- **Description:** eval()
- **Evidence (Line):** `raises(ValueError, lambda: motzkin.eval(-8))`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1269. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/elementary/complexes.py:69
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, arg):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1270. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/elementary/complexes.py:191
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, arg):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1271. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/elementary/complexes.py:330
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, arg):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1272. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/elementary/complexes.py:523
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, arg):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1273. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/elementary/complexes.py:750
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, arg):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1274. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/elementary/complexes.py:867
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, arg):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1275. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/elementary/complexes.py:939
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, arg):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1276. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/elementary/complexes.py:982
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, arg):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1277. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/elementary/complexes.py:1062
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, arg):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1278. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/elementary/complexes.py:1170
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, ar, period):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1279. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/elementary/complexes.py:1278
- **Description:** eval()
- **Evidence (Line):** `def eval(self, x, period):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1280. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/elementary/exponential.py:276
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, arg):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1281. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/elementary/exponential.py:661
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, arg, base=None):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1282. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/elementary/exponential.py:1146
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, x, k=None):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1283. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/elementary/hyperbolic.py:189
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, arg):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1284. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/elementary/hyperbolic.py:392
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, arg):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1285. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/elementary/hyperbolic.py:644
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, arg):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1286. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/elementary/hyperbolic.py:857
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, arg):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1287. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/elementary/hyperbolic.py:1005
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, arg):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1288. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/elementary/hyperbolic.py:1012
- **Description:** eval()
- **Evidence (Line):** `t = cls._reciprocal_of.eval(arg)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1289. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/elementary/hyperbolic.py:1235
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, arg):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1290. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/elementary/hyperbolic.py:1414
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, arg):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1291. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/elementary/hyperbolic.py:1599
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, arg):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1292. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/elementary/hyperbolic.py:1764
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, arg):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1293. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/elementary/hyperbolic.py:1930
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, arg):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1294. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/elementary/hyperbolic.py:2126
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, arg):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1295. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/elementary/integers.py:27
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, arg):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1296. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/elementary/integers.py:546
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, arg):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1297. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/elementary/integers.py:549
- **Description:** eval()
- **Evidence (Line):** `def _eval(arg):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1298. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/elementary/integers.py:576
- **Description:** eval()
- **Evidence (Line):** `real = _eval(real)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1299. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/elementary/integers.py:577
- **Description:** eval()
- **Evidence (Line):** `imag = _eval(imag)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1300. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/elementary/miscellaneous.py:901
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, p, q):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1301. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/elementary/piecewise.py:148
- **Description:** eval()
- **Evidence (Line):** `r = cls.eval(*newargs)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1302. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/elementary/piecewise.py:157
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, *_args):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1303. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/elementary/tests/test_interface.py:26
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, arg):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1304. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/elementary/tests/test_interface.py:47
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, arg):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1305. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/elementary/tests/test_interface.py:75
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, arg):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1306. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/elementary/tests/test_piecewise.py:1574
- **Description:** eval()
- **Evidence (Line):** `def test_piecewise_eval():`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1307. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/elementary/trigonometric.py:301
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, arg):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1308. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/elementary/trigonometric.py:607
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, arg):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1309. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/elementary/trigonometric.py:969
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, arg):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1310. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/elementary/trigonometric.py:1309
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, arg):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1311. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/elementary/trigonometric.py:1580
- **Description:** eval()
- **Evidence (Line):** `# trigonometric functions eval() like even/odd, func(x+2*k*pi), etc.`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1312. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/elementary/trigonometric.py:1587
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, arg):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1313. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/elementary/trigonometric.py:1613
- **Description:** eval()
- **Evidence (Line):** `t = cls._reciprocal_of.eval(arg)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1314. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/elementary/trigonometric.py:1974
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, arg):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1315. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/elementary/trigonometric.py:2174
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, arg):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1316. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/elementary/trigonometric.py:2402
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, arg):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1317. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/elementary/trigonometric.py:2654
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, arg):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1318. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/elementary/trigonometric.py:2866
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, arg):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1319. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/elementary/trigonometric.py:3076
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, arg):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1320. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/elementary/trigonometric.py:3290
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, arg):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1321. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/elementary/trigonometric.py:3552
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, y, x):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1322. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/bessel.py:63
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, nu, z):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1323. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/bessel.py:179
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, nu, z):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1324. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/bessel.py:327
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, nu, z):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1325. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/bessel.py:482
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, nu, z):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1326. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/bessel.py:649
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, nu, z):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1327. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/bessel.py:999
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, nu, z):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1328. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/bessel.py:1467
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, arg):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1329. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/bessel.py:1641
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, arg):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1330. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/bessel.py:1809
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, arg):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1331. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/bessel.py:1968
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, arg):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1332. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/bessel.py:2094
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, m, a, b):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1333. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/beta_functions.py:121
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, x, y=None):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1334. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/delta_functions.py:152
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, arg, k=S.Zero):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1335. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/delta_functions.py:160
- **Description:** eval()
- **Evidence (Line):** `The ``eval()`` method is automatically called when the ``DiracDelta```
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1336. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/delta_functions.py:163
- **Description:** eval()
- **Evidence (Line):** `In other words, ``eval()`` method is not needed to be called explicitly,`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1337. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/delta_functions.py:496
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, arg, H0=S.Half):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1338. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/delta_functions.py:504
- **Description:** eval()
- **Evidence (Line):** `The ``eval()`` method is automatically called when the ``Heaviside```
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1339. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/delta_functions.py:507
- **Description:** eval()
- **Evidence (Line):** `In other words, ``eval()`` method is not needed to be called explicitly,`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1340. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/elliptic_integrals.py:59
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, m):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1341. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/elliptic_integrals.py:146
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, z, m):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1342. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/elliptic_integrals.py:237
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, m, z=None):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1343. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/elliptic_integrals.py:356
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, n, m, z=None):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1344. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/error_functions.py:143
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, arg):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1345. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/error_functions.py:381
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, arg):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1346. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/error_functions.py:565
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, z):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1347. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/error_functions.py:754
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, x, y):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1348. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/error_functions.py:884
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, z):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1349. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/error_functions.py:971
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, z):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1350. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/error_functions.py:1057
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, x, y):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1351. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/error_functions.py:1181
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, z):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1352. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/error_functions.py:1380
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, nu, z):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1353. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/error_functions.py:1389
- **Description:** eval()
- **Evidence (Line):** `# explained in lowergamma.eval().`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1354. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/error_functions.py:1603
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, z):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1355. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/error_functions.py:1735
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, z):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1356. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/error_functions.py:1770
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, z):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1357. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/error_functions.py:2333
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, z):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1358. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/error_functions.py:2717
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, arg):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1359. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/gamma_functions.py:121
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, arg):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1360. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/gamma_functions.py:292
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, a, x):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1361. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/gamma_functions.py:485
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, a, z):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1362. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/gamma_functions.py:672
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, n, z):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1363. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/gamma_functions.py:976
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, z):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1364. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/gamma_functions.py:1135
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, z):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1365. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/gamma_functions.py:1229
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, z):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1366. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/gamma_functions.py:1324
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, x, p):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1367. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/hyper.py:226
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, ap, bq, z):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1368. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/hyper.py:814
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, *args):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1369. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/hyper.py:1165
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, a, b1, b2, c, x, y):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1370. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/mathieu_functions.py:78
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, a, q, z):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1371. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/mathieu_functions.py:140
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, a, q, z):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1372. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/mathieu_functions.py:202
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, a, q, z):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1373. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/mathieu_functions.py:264
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, a, q, z):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1374. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/polynomials.py:127
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, n, a, b, x):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1375. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/polynomials.py:360
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, n, a, x):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1376. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/polynomials.py:514
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, n, x):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1377. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/polynomials.py:632
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, n, x):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1378. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/polynomials.py:721
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, n, k):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1379. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/polynomials.py:762
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, n, k):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1380. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/polynomials.py:830
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, n, x):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1381. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/polynomials.py:954
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, n, m, x):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1382. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/polynomials.py:1063
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, n, x):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1383. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/polynomials.py:1169
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, n, x):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1384. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/polynomials.py:1270
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, n, x):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1385. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/polynomials.py:1392
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, n, alpha, x):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1386. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/singularity_functions.py:114
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, variable, offset, exponent):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1387. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/singularity_functions.py:122
- **Description:** eval()
- **Evidence (Line):** `The ``eval()`` method is automatically called when the`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1388. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/singularity_functions.py:125
- **Description:** eval()
- **Evidence (Line):** `depending on the argument passed. In other words, ``eval()`` method is`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1389. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/spherical_harmonics.py:138
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, n, m, theta, phi):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1390. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/spherical_harmonics.py:326
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, n, m, theta, phi):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1391. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/tensor_functions.py:80
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, *args):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1392. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/tensor_functions.py:147
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, i, j, delta_range=None):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1393. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/tests/test_bessel.py:458
- **Description:** eval()
- **Evidence (Line):** `def test_bessel_eval():`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1394. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/tests/test_hyper.py:332
- **Description:** eval()
- **Evidence (Line):** `def test_meijerg_eval():`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1395. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/tests/test_singularity_functions.py:44
- **Description:** eval()
- **Evidence (Line):** `def test_eval():`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1396. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/tests/test_zeta_functions.py:24
- **Description:** eval()
- **Evidence (Line):** `def test_zeta_eval():`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1397. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/tests/test_zeta_functions.py:87
- **Description:** eval()
- **Evidence (Line):** `def test_dirichlet_eta_eval():`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1398. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/tests/test_zeta_functions.py:99
- **Description:** eval()
- **Evidence (Line):** `def test_riemann_xi_eval():`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1399. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/zeta_functions.py:294
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, s, z):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1400. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/zeta_functions.py:503
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, s, a=None):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1401. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/zeta_functions.py:632
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, s, a=None):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1402. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/zeta_functions.py:692
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, s):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1403. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/functions/special/zeta_functions.py:744
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, n, a=None):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1404. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/manualintegrate.py:78
- **Description:** eval()
- **Evidence (Line):** `def eval(self) -> Expr:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1405. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/manualintegrate.py:96
- **Description:** eval()
- **Evidence (Line):** `def eval(self) -> Expr:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1406. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/manualintegrate.py:107
- **Description:** eval()
- **Evidence (Line):** `def eval(self) -> Expr:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1407. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/manualintegrate.py:108
- **Description:** eval()
- **Evidence (Line):** `return self.constant * self.substep.eval()`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1408. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/manualintegrate.py:120
- **Description:** eval()
- **Evidence (Line):** `def eval(self) -> Expr:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1409. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/manualintegrate.py:133
- **Description:** eval()
- **Evidence (Line):** `def eval(self) -> Expr:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1410. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/manualintegrate.py:144
- **Description:** eval()
- **Evidence (Line):** `def eval(self) -> Expr:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1411. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/manualintegrate.py:145
- **Description:** eval()
- **Evidence (Line):** `return Add(*(substep.eval() for substep in self.substeps))`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1412. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/manualintegrate.py:158
- **Description:** eval()
- **Evidence (Line):** `def eval(self) -> Expr:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1413. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/manualintegrate.py:159
- **Description:** eval()
- **Evidence (Line):** `result = self.substep.eval()`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1414. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/manualintegrate.py:179
- **Description:** eval()
- **Evidence (Line):** `def eval(self) -> Expr:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1415. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/manualintegrate.py:181
- **Description:** eval()
- **Evidence (Line):** `v = self.v_step.eval()`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1416. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/manualintegrate.py:182
- **Description:** eval()
- **Evidence (Line):** `return self.u * v - self.second_step.eval()`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1417. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/manualintegrate.py:195
- **Description:** eval()
- **Evidence (Line):** `def eval(self) -> Expr:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1418. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/manualintegrate.py:199
- **Description:** eval()
- **Evidence (Line):** `result.append(sign * rule.u * rule.v_step.eval())`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1419. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/manualintegrate.py:215
- **Description:** eval()
- **Evidence (Line):** `def eval(self) -> Expr:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1420. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/manualintegrate.py:222
- **Description:** eval()
- **Evidence (Line):** `def eval(self) -> Expr:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1421. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/manualintegrate.py:229
- **Description:** eval()
- **Evidence (Line):** `def eval(self) -> Expr:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1422. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/manualintegrate.py:236
- **Description:** eval()
- **Evidence (Line):** `def eval(self) -> Expr:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1423. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/manualintegrate.py:243
- **Description:** eval()
- **Evidence (Line):** `def eval(self) -> Expr:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1424. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/manualintegrate.py:250
- **Description:** eval()
- **Evidence (Line):** `def eval(self) -> Expr:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1425. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/manualintegrate.py:262
- **Description:** eval()
- **Evidence (Line):** `def eval(self) -> Expr:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1426. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/manualintegrate.py:269
- **Description:** eval()
- **Evidence (Line):** `def eval(self):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1427. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/manualintegrate.py:279
- **Description:** eval()
- **Evidence (Line):** `def eval(self) -> Expr:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1428. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/manualintegrate.py:288
- **Description:** eval()
- **Evidence (Line):** `def eval(self) -> Expr:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1429. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/manualintegrate.py:295
- **Description:** eval()
- **Evidence (Line):** `def eval(self) -> Expr:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1430. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/manualintegrate.py:302
- **Description:** eval()
- **Evidence (Line):** `def eval(self) -> Expr:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1431. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/manualintegrate.py:313
- **Description:** eval()
- **Evidence (Line):** `def eval(self) -> Expr:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1432. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/manualintegrate.py:326
- **Description:** eval()
- **Evidence (Line):** `def eval(self) -> Expr:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1433. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/manualintegrate.py:349
- **Description:** eval()
- **Evidence (Line):** `I0 = constant*step.eval()`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1434. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/manualintegrate.py:361
- **Description:** eval()
- **Evidence (Line):** `def eval(self) -> Expr:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1435. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/manualintegrate.py:363
- **Description:** eval()
- **Evidence (Line):** `return step.eval()`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1436. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/manualintegrate.py:371
- **Description:** eval()
- **Evidence (Line):** `def eval(self) -> Expr:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1437. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/manualintegrate.py:372
- **Description:** eval()
- **Evidence (Line):** `return self.alternatives[0].eval()`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1438. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/manualintegrate.py:381
- **Description:** eval()
- **Evidence (Line):** `def eval(self) -> Expr:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1439. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/manualintegrate.py:391
- **Description:** eval()
- **Evidence (Line):** `def eval(self) -> Expr:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1440. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/manualintegrate.py:407
- **Description:** eval()
- **Evidence (Line):** `def eval(self) -> Expr:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1441. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/manualintegrate.py:408
- **Description:** eval()
- **Evidence (Line):** `return self.substep.eval()`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1442. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/manualintegrate.py:424
- **Description:** eval()
- **Evidence (Line):** `def eval(self) -> Expr:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1443. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/manualintegrate.py:425
- **Description:** eval()
- **Evidence (Line):** `return Piecewise(*[(substep.eval(), cond)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1444. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/manualintegrate.py:438
- **Description:** eval()
- **Evidence (Line):** `def eval(self) -> Expr:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1445. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/manualintegrate.py:443
- **Description:** eval()
- **Evidence (Line):** `result = self.substep.eval()`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1446. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/manualintegrate.py:456
- **Description:** eval()
- **Evidence (Line):** `def eval(self) -> Expr:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1447. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/manualintegrate.py:471
- **Description:** eval()
- **Evidence (Line):** `def eval(self) -> Expr:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1448. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/manualintegrate.py:507
- **Description:** eval()
- **Evidence (Line):** `(self.substep.eval().subs(substitution).trigsimp(), self.restriction) # type: ignore`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1449. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/manualintegrate.py:521
- **Description:** eval()
- **Evidence (Line):** `def eval(self) -> Expr:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1450. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/manualintegrate.py:536
- **Description:** eval()
- **Evidence (Line):** `def eval(self) -> Expr:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1451. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/manualintegrate.py:548
- **Description:** eval()
- **Evidence (Line):** `def eval(self) -> Expr:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1452. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/manualintegrate.py:558
- **Description:** eval()
- **Evidence (Line):** `def eval(self) -> Expr:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1453. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/manualintegrate.py:568
- **Description:** eval()
- **Evidence (Line):** `def eval(self) -> Expr:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1454. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/manualintegrate.py:577
- **Description:** eval()
- **Evidence (Line):** `def eval(self) -> Expr:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1455. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/manualintegrate.py:584
- **Description:** eval()
- **Evidence (Line):** `def eval(self) -> Expr:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1456. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/manualintegrate.py:591
- **Description:** eval()
- **Evidence (Line):** `def eval(self) -> Expr:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1457. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/manualintegrate.py:600
- **Description:** eval()
- **Evidence (Line):** `def eval(self) -> Expr:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1458. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/manualintegrate.py:612
- **Description:** eval()
- **Evidence (Line):** `def eval(self) -> Expr:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1459. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/manualintegrate.py:619
- **Description:** eval()
- **Evidence (Line):** `def eval(self) -> Expr:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1460. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/manualintegrate.py:626
- **Description:** eval()
- **Evidence (Line):** `def eval(self) -> Expr:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1461. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/manualintegrate.py:633
- **Description:** eval()
- **Evidence (Line):** `def eval(self) -> Expr:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1462. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/manualintegrate.py:640
- **Description:** eval()
- **Evidence (Line):** `def eval(self) -> Expr:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1463. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/manualintegrate.py:647
- **Description:** eval()
- **Evidence (Line):** `def eval(self) -> Expr:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1464. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/manualintegrate.py:658
- **Description:** eval()
- **Evidence (Line):** `def eval(self) -> Expr:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1465. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/manualintegrate.py:676
- **Description:** eval()
- **Evidence (Line):** `def eval(self) -> Expr:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1466. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/manualintegrate.py:689
- **Description:** eval()
- **Evidence (Line):** `def eval(self) -> Expr:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1467. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/manualintegrate.py:701
- **Description:** eval()
- **Evidence (Line):** `def eval(self) -> Expr:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1468. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/manualintegrate.py:710
- **Description:** eval()
- **Evidence (Line):** `def eval(self) -> Expr:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1469. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/manualintegrate.py:720
- **Description:** eval()
- **Evidence (Line):** `def eval(self) -> Expr:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1470. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/manualintegrate.py:729
- **Description:** eval()
- **Evidence (Line):** `def eval(self) -> Expr:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1471. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/manualintegrate.py:1277
- **Description:** eval()
- **Evidence (Line):** `v = v_step.eval()`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1472. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/manualintegrate.py:1299
- **Description:** eval()
- **Evidence (Line):** `v = v_step.eval()`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1473. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/manualintegrate.py:2164
- **Description:** eval()
- **Evidence (Line):** `result = integral_steps(f, var).eval()`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1474. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/meijerint.py:113
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, arg):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1475. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/prde.py:138
- **Description:** eval()
- **Evidence (Line):** `alphaa, alphad = frac_in(-ba.eval(0)/bd.eval(0)/a.eval(0), DE.t)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1476. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/prde.py:138
- **Description:** eval()
- **Evidence (Line):** `alphaa, alphad = frac_in(-ba.eval(0)/bd.eval(0)/a.eval(0), DE.t)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1477. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/prde.py:138
- **Description:** eval()
- **Evidence (Line):** `alphaa, alphad = frac_in(-ba.eval(0)/bd.eval(0)/a.eval(0), DE.t)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1478. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/rde.py:233
- **Description:** eval()
- **Evidence (Line):** `alphaa, alphad = frac_in(-ba.eval(0)/bd.eval(0)/a.eval(0), DE.t)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1479. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/rde.py:233
- **Description:** eval()
- **Evidence (Line):** `alphaa, alphad = frac_in(-ba.eval(0)/bd.eval(0)/a.eval(0), DE.t)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1480. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/rde.py:233
- **Description:** eval()
- **Evidence (Line):** `alphaa, alphad = frac_in(-ba.eval(0)/bd.eval(0)/a.eval(0), DE.t)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1481. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/rde.py:245
- **Description:** eval()
- **Evidence (Line):** `alphaa, alphad = frac_in(im(-ba.eval(sqrt(-1))/bd.eval(sqrt(-1))/a.eval(sqrt(-1))), DE.t)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1482. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/rde.py:245
- **Description:** eval()
- **Evidence (Line):** `alphaa, alphad = frac_in(im(-ba.eval(sqrt(-1))/bd.eval(sqrt(-1))/a.eval(sqrt(-1))), DE.t)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1483. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/rde.py:245
- **Description:** eval()
- **Evidence (Line):** `alphaa, alphad = frac_in(im(-ba.eval(sqrt(-1))/bd.eval(sqrt(-1))/a.eval(sqrt(-1))), DE.t)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1484. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/rde.py:246
- **Description:** eval()
- **Evidence (Line):** `betaa, betad = frac_in(re(-ba.eval(sqrt(-1))/bd.eval(sqrt(-1))/a.eval(sqrt(-1))), DE.t)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1485. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/rde.py:246
- **Description:** eval()
- **Evidence (Line):** `betaa, betad = frac_in(re(-ba.eval(sqrt(-1))/bd.eval(sqrt(-1))/a.eval(sqrt(-1))), DE.t)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1486. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/rde.py:246
- **Description:** eval()
- **Evidence (Line):** `betaa, betad = frac_in(re(-ba.eval(sqrt(-1))/bd.eval(sqrt(-1))/a.eval(sqrt(-1))), DE.t)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1487. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/risch.py:630
- **Description:** eval()
- **Evidence (Line):** `# eval(repr(DE)) == DE, where DE is the DifferentialExtension object,`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1488. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/integrals/risch.py:1197
- **Description:** eval()
- **Evidence (Line):** `delta_a = delta_a*Poly((DE.t - alpha)**(n - j), DE.t) + Poly(H.eval(alpha), DE.t)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1489. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/interactive/session.py:105
- **Description:** exec()
- **Evidence (Line):** `>>> exec(s)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1490. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/interactive/session.py:107
- **Description:** exec()
- **Evidence (Line):** `>>> exec(int_to_Integer(s))`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1491. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/logic/boolalg.py:909
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, arg):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1492. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/logic/boolalg.py:1133
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, *args):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1493. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/logic/boolalg.py:1167
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, *args):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1494. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/logic/boolalg.py:1198
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, *args):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1495. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/logic/boolalg.py:1252
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, *args):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1496. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/logic/boolalg.py:1439
- **Description:** eval()
- **Evidence (Line):** `rv = cls.eval(a, b, c)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1497. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/logic/boolalg.py:1445
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, *args):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1498. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/logic/boolalg.py:1508
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, *args):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1499. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/matrices/tests/test_commonmatrix.py:825
- **Description:** eval()
- **Evidence (Line):** `eval('c = a @ b')`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1500. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/matrices/tests/test_commonmatrix.py:864
- **Description:** eval()
- **Evidence (Line):** `eval('c = 5 @ b')`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1501. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/matrices/tests/test_commonmatrix.py:894
- **Description:** eval()
- **Evidence (Line):** `eval('2 @ a')`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1502. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/matrices/tests/test_commonmatrix.py:902
- **Description:** eval()
- **Evidence (Line):** `eval('a @ 2')`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1503. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/matrices/tests/test_matrices.py:209
- **Description:** eval()
- **Evidence (Line):** `eval('c = a @ b')`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1504. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/matrices/tests/test_matrices.py:248
- **Description:** eval()
- **Evidence (Line):** `eval('c = 5 @ b')`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1505. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/matrices/tests/test_matrixbase.py:506
- **Description:** eval()
- **Evidence (Line):** `eval('2 @ a')`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1506. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/matrices/tests/test_matrixbase.py:514
- **Description:** eval()
- **Evidence (Line):** `eval('a @ 2')`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1507. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/matrices/tests/test_sparse.py:99
- **Description:** eval()
- **Evidence (Line):** `eval('c = a @ b')`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1508. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/parsing/ast_parser.py:79
- **Description:** eval()
- **Evidence (Line):** `return eval(e, global_dict, local_dict)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1509. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/parsing/ast_parser.py:72
- **Description:** exec()
- **Evidence (Line):** `exec('from sympy import *', global_dict)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1510. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/parsing/sympy_parser.py:905
- **Description:** eval()
- **Evidence (Line):** `expr = eval(`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1511. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/parsing/sympy_parser.py:1052
- **Description:** exec()
- **Evidence (Line):** `exec('from sympy import *', global_dict)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1512. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/parsing/tests/test_autolev.py:140
- **Description:** exec()
- **Evidence (Line):** `exec(sympy_input, g, l)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1513. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/parsing/tests/test_sympy_parser.py:156
- **Description:** exec()
- **Evidence (Line):** `exec('from sympy import *', default_globals)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1514. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/physics/biomechanics/curve.py:31
- **Description:** eval()
- **Evidence (Line):** `def eval(cls):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1515. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/physics/biomechanics/curve.py:192
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, l_T_tilde, c0, c1, c2, c3):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1516. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/physics/biomechanics/curve.py:411
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, fl_T, c0, c1, c2, c3):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1517. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/physics/biomechanics/curve.py:633
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, l_M_tilde, c0, c1):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1518. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/physics/biomechanics/curve.py:847
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, fl_M_pas, c0, c1):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1519. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/physics/biomechanics/curve.py:1103
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, l_M_tilde, c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1520. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/physics/biomechanics/curve.py:1407
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, v_M_tilde, c0, c1, c2, c3):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1521. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/physics/biomechanics/curve.py:1630
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, fv_M, c0, c1, c2, c3):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1522. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/physics/biomechanics/tests/test_curve.py:78
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, a, b):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1523. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/physics/control/control_plots.py:53
- **Description:** eval()
- **Evidence (Line):** `def _eval(l):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1524. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/physics/control/control_plots.py:56
- **Description:** eval()
- **Evidence (Line):** `return _eval(poly.all_roots())`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1525. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/physics/control/control_plots.py:59
- **Description:** eval()
- **Evidence (Line):** `return _eval(poly.nroots())`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1526. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/physics/quantum/anticommutator.py:92
- **Description:** eval()
- **Evidence (Line):** `r = cls.eval(A, B)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1527. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/physics/quantum/anticommutator.py:99
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, a, b):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1528. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/physics/quantum/commutator.py:106
- **Description:** eval()
- **Evidence (Line):** `r = cls.eval(A, B)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1529. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/physics/quantum/commutator.py:113
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, a, b):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1530. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/physics/quantum/hilbert.py:146
- **Description:** eval()
- **Evidence (Line):** `r = cls.eval(dimension)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1531. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/physics/quantum/hilbert.py:153
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, dimension):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1532. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/physics/quantum/hilbert.py:329
- **Description:** eval()
- **Evidence (Line):** `r = cls.eval(args)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1533. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/physics/quantum/hilbert.py:336
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, args):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1534. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/physics/quantum/hilbert.py:473
- **Description:** eval()
- **Evidence (Line):** `r = cls.eval(args)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1535. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/physics/quantum/hilbert.py:480
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, args):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1536. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/physics/quantum/hilbert.py:591
- **Description:** eval()
- **Evidence (Line):** `r = cls.eval(args)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1537. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/physics/quantum/hilbert.py:597
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, args):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1538. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/physics/quantum/state.py:812
- **Description:** eval()
- **Evidence (Line):** `def eval(self, *args):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1539. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/physics/quantum/tests/test_printing.py:62
- **Description:** eval()
- **Evidence (Line):** `assert eval(string, ENV) == expr`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1540. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/physics/quantum/tests/test_printing.py:45
- **Description:** exec()
- **Evidence (Line):** `exec('from sympy import *', ENV)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1541. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/physics/quantum/tests/test_printing.py:46
- **Description:** exec()
- **Evidence (Line):** `exec('from sympy.physics.quantum import *', ENV)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1542. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/physics/quantum/tests/test_printing.py:47
- **Description:** exec()
- **Evidence (Line):** `exec('from sympy.physics.quantum.cg import *', ENV)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1543. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/physics/quantum/tests/test_printing.py:48
- **Description:** exec()
- **Evidence (Line):** `exec('from sympy.physics.quantum.spin import *', ENV)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1544. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/physics/quantum/tests/test_printing.py:49
- **Description:** exec()
- **Evidence (Line):** `exec('from sympy.physics.quantum.hilbert import *', ENV)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1545. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/physics/quantum/tests/test_printing.py:50
- **Description:** exec()
- **Evidence (Line):** `exec('from sympy.physics.quantum.qubit import *', ENV)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1546. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/physics/quantum/tests/test_printing.py:51
- **Description:** exec()
- **Evidence (Line):** `exec('from sympy.physics.quantum.qexpr import *', ENV)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1547. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/physics/quantum/tests/test_printing.py:52
- **Description:** exec()
- **Evidence (Line):** `exec('from sympy.physics.quantum.gate import *', ENV)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1548. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/physics/quantum/tests/test_printing.py:53
- **Description:** exec()
- **Evidence (Line):** `exec('from sympy.physics.quantum.constants import *', ENV)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1549. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/physics/secondquant.py:114
- **Description:** eval()
- **Evidence (Line):** `r = cls.eval(arg)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1550. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/physics/secondquant.py:121
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, arg):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1551. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/physics/secondquant.py:137
- **Description:** eval()
- **Evidence (Line):** `The eval() method is called automatically.`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1552. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/physics/secondquant.py:1440
- **Description:** eval()
- **Evidence (Line):** `return cls.eval(bra, ket)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1553. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/physics/secondquant.py:1443
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, bra, ket):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1554. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/physics/secondquant.py:1601
- **Description:** eval()
- **Evidence (Line):** `result = eval(list_comp)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1555. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/physics/secondquant.py:1699
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, a, b):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1556. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/physics/secondquant.py:1710
- **Description:** eval()
- **Evidence (Line):** `>>> Commutator.eval(c1, c2)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1557. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/physics/tests/test_secondquant.py:273
- **Description:** eval()
- **Evidence (Line):** `assert Commutator.eval(c1, c1) == 0`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1558. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/physics/units/tests/test_prefixes.py:85
- **Description:** eval()
- **Evidence (Line):** `assert eval(repr(kilo)) == kilo`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1559. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/physics/units/tests/test_prefixes.py:86
- **Description:** eval()
- **Evidence (Line):** `assert eval(repr(kibi)) == kibi`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1560. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/physics/vector/tests/test_frame.py:759
- **Description:** pickle load/dumps
- **Evidence (Line):** `N1 = pickle.loads(pickle.dumps(N))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1561. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/plotting/experimental_lambdify.py:268
- **Description:** exec()
- **Evidence (Line):** `exec("MYNEWLAMBDA = %s" % eval_str, namespace)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1562. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/plotting/pygletplot/plot_axes.py:44
- **Description:** eval()
- **Evidence (Line):** `stride = eval(stride)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1563. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/plotting/pygletplot/plot_interval.py:27
- **Description:** eval()
- **Evidence (Line):** `args = eval(args[0])`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1564. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/plotting/series.py:44
- **Description:** eval()
- **Evidence (Line):** `def _uniform_eval(f1, f2, *args, modules=None,`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1565. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/plotting/series.py:91
- **Description:** eval()
- **Evidence (Line):** `def _adaptive_eval(f, x):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1566. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/plotting/series.py:479
- **Description:** eval()
- **Evidence (Line):** `r = _uniform_eval(*f, *args)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1567. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/plotting/series.py:1410
- **Description:** eval()
- **Evidence (Line):** `ynew = _adaptive_eval(f, xnew)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1568. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/plotting/series.py:1449
- **Description:** eval()
- **Evidence (Line):** `f_start = _adaptive_eval(f, self.start.real)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1569. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/plotting/series.py:1450
- **Description:** eval()
- **Evidence (Line):** `f_end = _adaptive_eval(f, self.end.real)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1570. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/plotting/series.py:1681
- **Description:** eval()
- **Evidence (Line):** `xnew = _adaptive_eval(f_x, param_new)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1571. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/plotting/series.py:1682
- **Description:** eval()
- **Evidence (Line):** `ynew = _adaptive_eval(f_y, param_new)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1572. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/plotting/series.py:1703
- **Description:** eval()
- **Evidence (Line):** `x_array = [_adaptive_eval(f_x, t) for t in param_array]`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1573. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/plotting/series.py:1704
- **Description:** eval()
- **Evidence (Line):** `y_array = [_adaptive_eval(f_y, t) for t in param_array]`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1574. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/plotting/series.py:1727
- **Description:** eval()
- **Evidence (Line):** `f_start_x = _adaptive_eval(f_x, self.start)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1575. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/plotting/series.py:1728
- **Description:** eval()
- **Evidence (Line):** `f_start_y = _adaptive_eval(f_y, self.start)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1576. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/plotting/series.py:1730
- **Description:** eval()
- **Evidence (Line):** `f_end_x = _adaptive_eval(f_x, self.end)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1577. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/plotting/series.py:1731
- **Description:** eval()
- **Evidence (Line):** `f_end_y = _adaptive_eval(f_y, self.end)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1578. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/plotting/series.py:2321
- **Description:** eval()
- **Evidence (Line):** `data = self._adaptive_eval()`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1579. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/plotting/series.py:2327
- **Description:** eval()
- **Evidence (Line):** `def _adaptive_eval(self):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1580. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/plotting/tests/test_series.py:1480
- **Description:** eval()
- **Evidence (Line):** `def test_complex_params_number_eval():`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1581. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/plotting/tests/test_series.py:1586
- **Description:** eval()
- **Evidence (Line):** `def test_force_real_eval():`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1582. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/appellseqs.py:38
- **Description:** eval()
- **Evidence (Line):** `p = dup_sub_ground(p, dup_eval(p, K(1,2), K) * K(1<<(i-1), (1<<i)-1), K)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1583. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/appellseqs.py:108
- **Description:** eval()
- **Evidence (Line):** `p = dup_sub_ground(p, dup_eval(p, K.one, K) * K((1<<(i-1))-1, (1<<i)-1), K)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1584. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/appellseqs.py:141
- **Description:** eval()
- **Evidence (Line):** `p = dup_sub_ground(p, dup_eval(p, K.one, K) // K(2), K)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1585. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/appellseqs.py:207
- **Description:** eval()
- **Evidence (Line):** `p = dup_sub_ground(p, dup_eval(p, K.one, K), K)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1586. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/compatibility.py:481
- **Description:** eval()
- **Evidence (Line):** `def dup_eval(self, f, a):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1587. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/compatibility.py:482
- **Description:** eval()
- **Evidence (Line):** `return dup_eval(self.to_dense(f), a, self.domain)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1588. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/compatibility.py:483
- **Description:** eval()
- **Evidence (Line):** `def dmp_eval(self, f, a):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1589. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/compatibility.py:484
- **Description:** eval()
- **Evidence (Line):** `result = dmp_eval(self.to_dense(f), a, self.ngens-1, self.domain)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1590. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/compatibility.py:1082
- **Description:** eval()
- **Evidence (Line):** `def gf_eval(self, f, a):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1591. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/compatibility.py:1083
- **Description:** eval()
- **Evidence (Line):** `return gf_eval(self.to_gf_dense(f), a, self.domain.mod, self.domain.dom)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1592. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/compatibility.py:1084
- **Description:** eval()
- **Evidence (Line):** `def gf_multi_eval(self, f, A):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1593. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/compatibility.py:1085
- **Description:** eval()
- **Evidence (Line):** `return gf_multi_eval(self.to_gf_dense(f), A, self.domain.mod, self.domain.dom)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1594. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/densetools.py:262
- **Description:** eval()
- **Evidence (Line):** `def dup_eval(f, a, K):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1595. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/densetools.py:272
- **Description:** eval()
- **Evidence (Line):** `>>> R.dup_eval(x**2 + 2*x + 3, 2)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1596. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/densetools.py:288
- **Description:** eval()
- **Evidence (Line):** `def dmp_eval(f, a, u, K):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1597. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/densetools.py:298
- **Description:** eval()
- **Evidence (Line):** `>>> R.dmp_eval(2*x*y + 3*x + y + 2, 2)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1598. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/densetools.py:303
- **Description:** eval()
- **Evidence (Line):** `return dup_eval(f, a, K)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1599. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/densetools.py:320
- **Description:** eval()
- **Evidence (Line):** `return dmp_eval(g, a, v, K)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1600. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/densetools.py:354
- **Description:** eval()
- **Evidence (Line):** `return dup_eval(g, A[-1], K)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1601. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/densetools.py:361
- **Description:** eval()
- **Evidence (Line):** `return dup_eval(h, A[-u + i - 1], K)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1602. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/densetools.py:396
- **Description:** eval()
- **Evidence (Line):** `def _rec_diff_eval(g, m, a, v, i, j, K):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1603. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/densetools.py:399
- **Description:** eval()
- **Evidence (Line):** `return dmp_eval(dmp_diff(g, m, v, K), a, v, K)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1604. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/densetools.py:403
- **Description:** eval()
- **Evidence (Line):** `return dmp_strip([ _rec_diff_eval(c, m, a, v, i, j, K) for c in g ], v)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1605. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/densetools.py:427
- **Description:** eval()
- **Evidence (Line):** `return dmp_eval(dmp_diff(f, m, u, K), a, u, K)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1606. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/densetools.py:429
- **Description:** eval()
- **Evidence (Line):** `return _rec_diff_eval(f, m, a, u, 0, j, K)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1607. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/densetools.py:991
- **Description:** eval()
- **Evidence (Line):** `return dup_strip([dup_eval(f, dup_LC(g, K), K)])`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1608. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/euclidtools.py:641
- **Description:** eval()
- **Evidence (Line):** `e = dmp_eval(r, a, v, K)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1609. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/euclidtools.py:650
- **Description:** eval()
- **Evidence (Line):** `d = K.invert(dup_eval(D, a, K), p)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1610. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/euclidtools.py:1205
- **Description:** eval()
- **Evidence (Line):** `ff = dup_eval(f, x, K)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1611. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/euclidtools.py:1206
- **Description:** eval()
- **Evidence (Line):** `gg = dup_eval(g, x, K)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1612. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/euclidtools.py:1330
- **Description:** eval()
- **Evidence (Line):** `ff = dmp_eval(f, x, u, K)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1613. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/euclidtools.py:1331
- **Description:** eval()
- **Evidence (Line):** `gg = dmp_eval(g, x, u, K)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1614. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/galoistools.py:1228
- **Description:** eval()
- **Evidence (Line):** `def gf_eval(f, a, p, K):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1615. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/galoistools.py:1238
- **Description:** eval()
- **Evidence (Line):** `>>> gf_eval([3, 2, 4], 2, 5, ZZ)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1616. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/galoistools.py:1252
- **Description:** eval()
- **Evidence (Line):** `def gf_multi_eval(f, A, p, K):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1617. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/galoistools.py:1262
- **Description:** eval()
- **Evidence (Line):** `>>> gf_multi_eval([3, 2, 4], [0, 1, 2, 3, 4], 5, ZZ)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1618. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/galoistools.py:1266
- **Description:** eval()
- **Evidence (Line):** `return [ gf_eval(f, a, p, K) for a in A ]`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1619. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/galoistools.py:1284
- **Description:** eval()
- **Evidence (Line):** `return gf_strip([gf_eval(f, gf_LC(g, K), p, K)])`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1620. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/galoistools.py:2478
- **Description:** eval()
- **Evidence (Line):** `X1 = root_zero + [i for i in range(p) if gf_eval(g, i, p, ZZ) == 0]`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1621. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/matrices/tests/test_domainmatrix.py:1366
- **Description:** pickle load/dumps
- **Evidence (Line):** `assert pickle.loads(pickle.dumps(dM)) == dM`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1622. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/matrices/tests/test_domainmatrix.py:1368
- **Description:** pickle load/dumps
- **Evidence (Line):** `assert pickle.loads(pickle.dumps(dM)) == dM`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1623. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/monomials.py:401
- **Description:** exec()
- **Evidence (Line):** `exec(code, ns)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1624. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/numberfields/galois_resolvents.py:423
- **Description:** eval()
- **Evidence (Line):** `if not dup_eval(R, (a := self.round_mpf(r.real)), ZZ):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1625. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/numberfields/galoisgroups.py:350
- **Description:** eval()
- **Evidence (Line):** `if not dup_eval(R51_dup, candidate_root, ZZ):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1626. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/numberfields/subfield.py:395
- **Description:** eval()
- **Evidence (Line):** `reps = [dup_eval(_.to_list(), ogen, K) for _ in reps] + [erep]`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1627. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/polyclasses.py:731
- **Description:** eval()
- **Evidence (Line):** `def eval(f, a, j=0):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1628. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/polyclasses.py:741
- **Description:** eval()
- **Evidence (Line):** `return f._eval(a)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1629. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/polyclasses.py:743
- **Description:** eval()
- **Evidence (Line):** `def _eval(f, a):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1630. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/polyclasses.py:1483
- **Description:** eval()
- **Evidence (Line):** `def _eval(f, a):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1631. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/polyclasses.py:2072
- **Description:** eval()
- **Evidence (Line):** `def _eval(f, a):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1632. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/polyclasses.py:2077
- **Description:** eval()
- **Evidence (Line):** `return f.to_DMP_Python()._eval(a)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1633. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/polyconfig.py:61
- **Description:** eval()
- **Evidence (Line):** `_current_config[key] = eval(value)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1634. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/polyquinticconst.py:159
- **Description:** eval()
- **Evidence (Line):** `l0 = Poly(a, x).eval(theta)/F`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1635. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/polyquinticconst.py:167
- **Description:** eval()
- **Evidence (Line):** `T[1] = -Poly(b[1], x).eval(theta)/(2*F)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1636. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/polyquinticconst.py:168
- **Description:** eval()
- **Evidence (Line):** `T[2] = Poly(b[2], x).eval(theta)/(2*d*F)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1637. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/polyquinticconst.py:169
- **Description:** eval()
- **Evidence (Line):** `T[3] = Poly(b[3], x).eval(theta)/(2*F)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1638. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/polyquinticconst.py:170
- **Description:** eval()
- **Evidence (Line):** `T[4] = Poly(b[4], x).eval(theta)/(2*d*F)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1639. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/polyquinticconst.py:176
- **Description:** eval()
- **Evidence (Line):** `order = Poly(o, x).eval(theta)/(d*F)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1640. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/polyquinticconst.py:182
- **Description:** eval()
- **Evidence (Line):** `v = Poly(c, x).eval(theta)/(2*d*self.F)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1641. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/polyroots.py:797
- **Description:** eval()
- **Evidence (Line):** `poly = poly.eval(gen, 1)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1642. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/polyroots.py:1037
- **Description:** eval()
- **Evidence (Line):** `if not f.eval(i):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1643. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/polytools.py:151
- **Description:** eval()
- **Evidence (Line):** `>>> Poly(y*x**2 + x*y + 1,x).eval(2)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1644. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/polytools.py:607
- **Description:** eval()
- **Evidence (Line):** `return f.eval(old, new)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1645. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/polytools.py:2419
- **Description:** eval()
- **Evidence (Line):** `def eval(self, x, a=None, auto=True):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1646. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/polytools.py:2429
- **Description:** eval()
- **Evidence (Line):** `>>> Poly(x**2 + 2*x + 3, x).eval(2)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1647. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/polytools.py:2432
- **Description:** eval()
- **Evidence (Line):** `>>> Poly(2*x*y + 3*x + y + 2, x, y).eval(x, 2)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1648. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/polytools.py:2437
- **Description:** eval()
- **Evidence (Line):** `>>> f.eval({x: 2})`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1649. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/polytools.py:2439
- **Description:** eval()
- **Evidence (Line):** `>>> f.eval({x: 2, y: 5})`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1650. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/polytools.py:2441
- **Description:** eval()
- **Evidence (Line):** `>>> f.eval({x: 2, y: 5, z: 7})`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1651. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/polytools.py:2444
- **Description:** eval()
- **Evidence (Line):** `>>> f.eval((2, 5))`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1652. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/polytools.py:2457
- **Description:** eval()
- **Evidence (Line):** `f = f.eval(gen, value)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1653. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/polytools.py:2467
- **Description:** eval()
- **Evidence (Line):** `f = f.eval(gen, value)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1654. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/polytools.py:2479
- **Description:** eval()
- **Evidence (Line):** `result = f.rep.eval(a, j)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1655. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/polytools.py:2490
- **Description:** eval()
- **Evidence (Line):** `result = f.rep.eval(a, j)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1656. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/polytools.py:2514
- **Description:** eval()
- **Evidence (Line):** `return f.eval(values)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1657. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/ring_series.py:2011
- **Description:** eval()
- **Evidence (Line):** `series = eval(_convert_func[str(expr.func)])(series_inner,`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1658. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/rootisolation.py:245
- **Description:** eval()
- **Evidence (Line):** `if not dup_eval(f, K.zero, K):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1659. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/rootisolation.py:252
- **Description:** eval()
- **Evidence (Line):** `if not dup_eval(f, K.zero, K):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1660. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/rootisolation.py:262
- **Description:** eval()
- **Evidence (Line):** `if not dup_eval(f, K.zero, K):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1661. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/rootisolation.py:786
- **Description:** eval()
- **Evidence (Line):** `signs_inf = dup_sign_variations([ dup_eval(s, inf, K) for s in sturm ], K)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1662. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/rootisolation.py:791
- **Description:** eval()
- **Evidence (Line):** `signs_sup = dup_sign_variations([ dup_eval(s, sup, K) for s in sturm ], K)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1663. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/rootisolation.py:795
- **Description:** eval()
- **Evidence (Line):** `if inf is not None and not dup_eval(f, inf, K):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1664. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/rootisolation.py:1048
- **Description:** eval()
- **Evidence (Line):** `if dup_eval(f2, t, F) > 0:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1665. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/rootisolation.py:1055
- **Description:** eval()
- **Evidence (Line):** `if dup_eval(f2, (s + a)/2, F) > 0:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1666. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/rootisolation.py:1064
- **Description:** eval()
- **Evidence (Line):** `if dup_eval(f2, s, F) > 0:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1667. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/rootisolation.py:1090
- **Description:** eval()
- **Evidence (Line):** `if dup_eval(f1, t, F) > 0:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1668. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/rootisolation.py:1097
- **Description:** eval()
- **Evidence (Line):** `if dup_eval(f1, (s + a)/2, F) > 0:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1669. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/rootisolation.py:1106
- **Description:** eval()
- **Evidence (Line):** `if dup_eval(f1, s, F) > 0:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1670. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/rootisolation.py:1127
- **Description:** eval()
- **Evidence (Line):** `re = dup_eval(f1, s, F)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1671. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/rootisolation.py:1128
- **Description:** eval()
- **Evidence (Line):** `im = dup_eval(f2, s, F)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1672. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/rootisolation.py:1134
- **Description:** eval()
- **Evidence (Line):** `re = dup_eval(f1, t, F)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1673. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/rootisolation.py:1135
- **Description:** eval()
- **Evidence (Line):** `im = dup_eval(f2, t, F)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1674. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/rootisolation.py:1139
- **Description:** eval()
- **Evidence (Line):** `re = dup_eval(f1, (s + a)/2, F)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1675. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/rootisolation.py:1140
- **Description:** eval()
- **Evidence (Line):** `im = dup_eval(f2, (s + a)/2, F)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1676. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/rootisolation.py:1165
- **Description:** eval()
- **Evidence (Line):** `re = dup_eval(f1, a, F)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1677. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/rootisolation.py:1166
- **Description:** eval()
- **Evidence (Line):** `im = dup_eval(f2, a, F)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1678. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/rootoftools.py:310
- **Description:** eval()
- **Evidence (Line):** ```eval(srepr(expr) == expr``.`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1679. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_densetools.py:191
- **Description:** eval()
- **Evidence (Line):** `def test_dup_eval():`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1680. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_densetools.py:192
- **Description:** eval()
- **Evidence (Line):** `assert dup_eval([], 7, ZZ) == 0`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1681. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_densetools.py:193
- **Description:** eval()
- **Evidence (Line):** `assert dup_eval([1, 2], 0, ZZ) == 2`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1682. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_densetools.py:194
- **Description:** eval()
- **Evidence (Line):** `assert dup_eval([1, 2, 3], 7, ZZ) == 66`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1683. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_densetools.py:197
- **Description:** eval()
- **Evidence (Line):** `def test_dmp_eval():`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1684. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_densetools.py:198
- **Description:** eval()
- **Evidence (Line):** `assert dmp_eval([], 3, 0, ZZ) == 0`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1685. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_densetools.py:200
- **Description:** eval()
- **Evidence (Line):** `assert dmp_eval([[]], 3, 1, ZZ) == []`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1686. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_densetools.py:201
- **Description:** eval()
- **Evidence (Line):** `assert dmp_eval([[[]]], 3, 2, ZZ) == [[]]`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1687. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_densetools.py:203
- **Description:** eval()
- **Evidence (Line):** `assert dmp_eval([[1, 2]], 0, 1, ZZ) == [1, 2]`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1688. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_densetools.py:205
- **Description:** eval()
- **Evidence (Line):** `assert dmp_eval([[[1]]], 3, 2, ZZ) == [[1]]`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1689. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_densetools.py:206
- **Description:** eval()
- **Evidence (Line):** `assert dmp_eval([[[1, 2]]], 3, 2, ZZ) == [[1, 2]]`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1690. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_densetools.py:208
- **Description:** eval()
- **Evidence (Line):** `assert dmp_eval([[3, 2], [1, 2]], 3, 1, ZZ) == [10, 8]`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1691. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_densetools.py:209
- **Description:** eval()
- **Evidence (Line):** `assert dmp_eval([[[3, 2]], [[1, 2]]], 3, 2, ZZ) == [[10, 8]]`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1692. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_densetools.py:214
- **Description:** eval()
- **Evidence (Line):** `f_6, -2, 1, 3, ZZ) == dmp_eval(dmp_swap(f_6, 0, 1, 3, ZZ), -2, 3, ZZ)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1693. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_densetools.py:216
- **Description:** eval()
- **Evidence (Line):** `f_6, 7, 1, 3, ZZ) == dmp_eval(dmp_swap(f_6, 0, 1, 3, ZZ), 7, 3, ZZ)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1694. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_densetools.py:218
- **Description:** eval()
- **Evidence (Line):** `dmp_eval(dmp_swap(f_6, 0, 2, 3, ZZ), -2, 3, ZZ), 0, 1, 2, ZZ)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1695. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_densetools.py:220
- **Description:** eval()
- **Evidence (Line):** `dmp_eval(dmp_swap(f_6, 0, 2, 3, ZZ), 7, 3, ZZ), 0, 1, 2, ZZ)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1696. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_densetools.py:259
- **Description:** eval()
- **Evidence (Line):** `dmp_eval(dmp_diff(dmp_swap(f_6, 0, 1, 3, ZZ), 2, 3, ZZ), 7, 3, ZZ)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1697. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_densetools.py:262
- **Description:** eval()
- **Evidence (Line):** `dmp_eval(dmp_diff(f_6, 2, 3, ZZ), 7, 3, ZZ)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1698. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_galoistools.py:381
- **Description:** eval()
- **Evidence (Line):** `def test_gf_eval():`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1699. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_galoistools.py:382
- **Description:** eval()
- **Evidence (Line):** `assert gf_eval([], 4, 11, ZZ) == 0`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1700. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_galoistools.py:383
- **Description:** eval()
- **Evidence (Line):** `assert gf_eval([], 27, 11, ZZ) == 0`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1701. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_galoistools.py:384
- **Description:** eval()
- **Evidence (Line):** `assert gf_eval([7], 4, 11, ZZ) == 7`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1702. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_galoistools.py:385
- **Description:** eval()
- **Evidence (Line):** `assert gf_eval([7], 27, 11, ZZ) == 7`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1703. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_galoistools.py:387
- **Description:** eval()
- **Evidence (Line):** `assert gf_eval([1, 0, 3, 2, 4, 3, 1, 2, 0], 0, 11, ZZ) == 0`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1704. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_galoistools.py:388
- **Description:** eval()
- **Evidence (Line):** `assert gf_eval([1, 0, 3, 2, 4, 3, 1, 2, 0], 4, 11, ZZ) == 9`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1705. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_galoistools.py:389
- **Description:** eval()
- **Evidence (Line):** `assert gf_eval([1, 0, 3, 2, 4, 3, 1, 2, 0], 27, 11, ZZ) == 5`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1706. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_galoistools.py:391
- **Description:** eval()
- **Evidence (Line):** `assert gf_eval([4, 0, 0, 4, 6, 0, 1, 3, 5], 0, 11, ZZ) == 5`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1707. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_galoistools.py:392
- **Description:** eval()
- **Evidence (Line):** `assert gf_eval([4, 0, 0, 4, 6, 0, 1, 3, 5], 4, 11, ZZ) == 3`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1708. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_galoistools.py:393
- **Description:** eval()
- **Evidence (Line):** `assert gf_eval([4, 0, 0, 4, 6, 0, 1, 3, 5], 27, 11, ZZ) == 9`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1709. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_galoistools.py:395
- **Description:** eval()
- **Evidence (Line):** `assert gf_multi_eval([3, 2, 1], [0, 1, 2, 3], 11, ZZ) == [1, 6, 6, 1]`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1710. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_injections.py:9
- **Description:** exec()
- **Evidence (Line):** `exec('R = vring("r", QQ)', ns)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1711. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_injections.py:10
- **Description:** exec()
- **Evidence (Line):** `exec('assert r == R.gens[0]', ns)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1712. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_injections.py:12
- **Description:** exec()
- **Evidence (Line):** `exec('R = vring("rb rbb rcc rzz _rx", QQ)', ns)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1713. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_injections.py:13
- **Description:** exec()
- **Evidence (Line):** `exec('assert rb == R.gens[0]', ns)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1714. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_injections.py:14
- **Description:** exec()
- **Evidence (Line):** `exec('assert rbb == R.gens[1]', ns)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1715. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_injections.py:15
- **Description:** exec()
- **Evidence (Line):** `exec('assert rcc == R.gens[2]', ns)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1716. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_injections.py:16
- **Description:** exec()
- **Evidence (Line):** `exec('assert rzz == R.gens[3]', ns)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1717. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_injections.py:17
- **Description:** exec()
- **Evidence (Line):** `exec('assert _rx == R.gens[4]', ns)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1718. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_injections.py:19
- **Description:** exec()
- **Evidence (Line):** `exec('R = vring(["rd", "re", "rfg"], QQ)', ns)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1719. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_injections.py:20
- **Description:** exec()
- **Evidence (Line):** `exec('assert rd == R.gens[0]', ns)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1720. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_injections.py:21
- **Description:** exec()
- **Evidence (Line):** `exec('assert re == R.gens[1]', ns)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1721. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_injections.py:22
- **Description:** exec()
- **Evidence (Line):** `exec('assert rfg == R.gens[2]', ns)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1722. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_injections.py:26
- **Description:** exec()
- **Evidence (Line):** `exec('F = vfield("f", QQ)', ns)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1723. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_injections.py:27
- **Description:** exec()
- **Evidence (Line):** `exec('assert f == F.gens[0]', ns)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1724. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_injections.py:29
- **Description:** exec()
- **Evidence (Line):** `exec('F = vfield("fb fbb fcc fzz _fx", QQ)', ns)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1725. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_injections.py:30
- **Description:** exec()
- **Evidence (Line):** `exec('assert fb == F.gens[0]', ns)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1726. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_injections.py:31
- **Description:** exec()
- **Evidence (Line):** `exec('assert fbb == F.gens[1]', ns)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1727. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_injections.py:32
- **Description:** exec()
- **Evidence (Line):** `exec('assert fcc == F.gens[2]', ns)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1728. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_injections.py:33
- **Description:** exec()
- **Evidence (Line):** `exec('assert fzz == F.gens[3]', ns)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1729. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_injections.py:34
- **Description:** exec()
- **Evidence (Line):** `exec('assert _fx == F.gens[4]', ns)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1730. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_injections.py:36
- **Description:** exec()
- **Evidence (Line):** `exec('F = vfield(["fd", "fe", "ffg"], QQ)', ns)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1731. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_injections.py:37
- **Description:** exec()
- **Evidence (Line):** `exec('assert fd == F.gens[0]', ns)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1732. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_injections.py:38
- **Description:** exec()
- **Evidence (Line):** `exec('assert fe == F.gens[1]', ns)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1733. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_injections.py:39
- **Description:** exec()
- **Evidence (Line):** `exec('assert ffg == F.gens[2]', ns)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1734. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_polyclasses.py:218
- **Description:** eval()
- **Evidence (Line):** `assert f.eval(a=1, j=0) == u`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1735. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_polyclasses.py:219
- **Description:** eval()
- **Evidence (Line):** `assert f.eval(a=1, j=1) == v`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1736. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_polyclasses.py:221
- **Description:** eval()
- **Evidence (Line):** `assert f.eval(1).eval(1) == ZZ(4)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1737. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_polyclasses.py:221
- **Description:** eval()
- **Evidence (Line):** `assert f.eval(1).eval(1) == ZZ(4)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1738. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_polytools.py:1639
- **Description:** eval()
- **Evidence (Line):** `def test_Poly_eval():`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1739. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_polytools.py:1640
- **Description:** eval()
- **Evidence (Line):** `assert Poly(0, x).eval(7) == 0`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1740. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_polytools.py:1641
- **Description:** eval()
- **Evidence (Line):** `assert Poly(1, x).eval(7) == 1`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1741. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_polytools.py:1642
- **Description:** eval()
- **Evidence (Line):** `assert Poly(x, x).eval(7) == 7`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1742. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_polytools.py:1644
- **Description:** eval()
- **Evidence (Line):** `assert Poly(0, x).eval(0, 7) == 0`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1743. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_polytools.py:1645
- **Description:** eval()
- **Evidence (Line):** `assert Poly(1, x).eval(0, 7) == 1`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1744. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_polytools.py:1646
- **Description:** eval()
- **Evidence (Line):** `assert Poly(x, x).eval(0, 7) == 7`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1745. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_polytools.py:1648
- **Description:** eval()
- **Evidence (Line):** `assert Poly(0, x).eval(x, 7) == 0`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1746. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_polytools.py:1649
- **Description:** eval()
- **Evidence (Line):** `assert Poly(1, x).eval(x, 7) == 1`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1747. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_polytools.py:1650
- **Description:** eval()
- **Evidence (Line):** `assert Poly(x, x).eval(x, 7) == 7`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1748. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_polytools.py:1652
- **Description:** eval()
- **Evidence (Line):** `assert Poly(0, x).eval('x', 7) == 0`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1749. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_polytools.py:1653
- **Description:** eval()
- **Evidence (Line):** `assert Poly(1, x).eval('x', 7) == 1`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1750. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_polytools.py:1654
- **Description:** eval()
- **Evidence (Line):** `assert Poly(x, x).eval('x', 7) == 7`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1751. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_polytools.py:1656
- **Description:** eval()
- **Evidence (Line):** `raises(PolynomialError, lambda: Poly(1, x).eval(1, 7))`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1752. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_polytools.py:1657
- **Description:** eval()
- **Evidence (Line):** `raises(PolynomialError, lambda: Poly(1, x).eval(y, 7))`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1753. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_polytools.py:1658
- **Description:** eval()
- **Evidence (Line):** `raises(PolynomialError, lambda: Poly(1, x).eval('y', 7))`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1754. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_polytools.py:1660
- **Description:** eval()
- **Evidence (Line):** `assert Poly(123, x, y).eval(7) == Poly(123, y)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1755. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_polytools.py:1661
- **Description:** eval()
- **Evidence (Line):** `assert Poly(2*y, x, y).eval(7) == Poly(2*y, y)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1756. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_polytools.py:1662
- **Description:** eval()
- **Evidence (Line):** `assert Poly(x*y, x, y).eval(7) == Poly(7*y, y)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1757. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_polytools.py:1664
- **Description:** eval()
- **Evidence (Line):** `assert Poly(123, x, y).eval(x, 7) == Poly(123, y)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1758. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_polytools.py:1665
- **Description:** eval()
- **Evidence (Line):** `assert Poly(2*y, x, y).eval(x, 7) == Poly(2*y, y)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1759. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_polytools.py:1666
- **Description:** eval()
- **Evidence (Line):** `assert Poly(x*y, x, y).eval(x, 7) == Poly(7*y, y)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1760. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_polytools.py:1668
- **Description:** eval()
- **Evidence (Line):** `assert Poly(123, x, y).eval(y, 7) == Poly(123, x)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1761. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_polytools.py:1669
- **Description:** eval()
- **Evidence (Line):** `assert Poly(2*y, x, y).eval(y, 7) == Poly(14, x)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1762. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_polytools.py:1670
- **Description:** eval()
- **Evidence (Line):** `assert Poly(x*y, x, y).eval(y, 7) == Poly(7*x, x)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1763. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_polytools.py:1672
- **Description:** eval()
- **Evidence (Line):** `assert Poly(x*y + y, x, y).eval({x: 7}) == Poly(8*y, y)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1764. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_polytools.py:1673
- **Description:** eval()
- **Evidence (Line):** `assert Poly(x*y + y, x, y).eval({y: 7}) == Poly(7*x + 7, x)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1765. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_polytools.py:1675
- **Description:** eval()
- **Evidence (Line):** `assert Poly(x*y + y, x, y).eval({x: 6, y: 7}) == 49`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1766. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_polytools.py:1676
- **Description:** eval()
- **Evidence (Line):** `assert Poly(x*y + y, x, y).eval({x: 7, y: 6}) == 48`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1767. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_polytools.py:1678
- **Description:** eval()
- **Evidence (Line):** `assert Poly(x*y + y, x, y).eval((6, 7)) == 49`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1768. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_polytools.py:1679
- **Description:** eval()
- **Evidence (Line):** `assert Poly(x*y + y, x, y).eval([6, 7]) == 49`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1769. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_polytools.py:1681
- **Description:** eval()
- **Evidence (Line):** `assert Poly(x + 1, domain='ZZ').eval(S.Half) == Rational(3, 2)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1770. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_polytools.py:1682
- **Description:** eval()
- **Evidence (Line):** `assert Poly(x + 1, domain='ZZ').eval(sqrt(2)) == sqrt(2) + 1`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1771. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_polytools.py:1684
- **Description:** eval()
- **Evidence (Line):** `raises(ValueError, lambda: Poly(x*y + y, x, y).eval((6, 7, 8)))`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1772. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_polytools.py:1685
- **Description:** eval()
- **Evidence (Line):** `raises(DomainError, lambda: Poly(x + 1, domain='ZZ').eval(S.Half, auto=False))`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1773. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_polytools.py:1692
- **Description:** eval()
- **Evidence (Line):** `assert f.eval((z + 1)/(z - 1)) == result`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1774. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_polytools.py:1695
- **Description:** eval()
- **Evidence (Line):** `assert g.eval((z + 1)/(z - 1)) == Poly(result, y, domain='ZZ(alpha,z)')`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1775. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_polytools.py:2834
- **Description:** eval()
- **Evidence (Line):** `def test_factor_noeval():`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1776. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/polys/tests/test_polytools.py:3960
- **Description:** pickle load/dumps
- **Evidence (Line):** `deserialized = pickle.loads(pickle.dumps(poly))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1777. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/printing/dot.py:43
- **Description:** eval()
- **Evidence (Line):** `>>> eval(code) == Symbol('x')`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1778. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/printing/dot.py:54
- **Description:** eval()
- **Evidence (Line):** `>>> eval(code) == MatrixSymbol('x', 2, 2)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1779. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/printing/python.py:44
- **Description:** exec()
- **Evidence (Line):** `(can be passed to the exec() function without any modifications)"""`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1780. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/printing/repr.py:5
- **Description:** eval()
- **Evidence (Line):** `relation eval(srepr(expr))=expr holds in an appropriate environment.`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1781. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/printing/tests/test_lambdarepr.py:36
- **Description:** eval()
- **Evidence (Line):** `# In each case, test eval() the lambdarepr() to make sure there are a`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1782. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/printing/tests/test_lambdarepr.py:43
- **Description:** eval()
- **Evidence (Line):** `eval(h + l)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1783. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/printing/tests/test_lambdarepr.py:52
- **Description:** eval()
- **Evidence (Line):** `eval(h + l)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1784. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/printing/tests/test_lambdarepr.py:60
- **Description:** eval()
- **Evidence (Line):** `eval(h + l)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1785. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/printing/tests/test_lambdarepr.py:69
- **Description:** eval()
- **Evidence (Line):** `eval(h + l)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1786. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/printing/tests/test_lambdarepr.py:79
- **Description:** eval()
- **Evidence (Line):** `eval(h + l)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1787. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/printing/tests/test_lambdarepr.py:89
- **Description:** eval()
- **Evidence (Line):** `eval(h + l)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1788. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/printing/tests/test_lambdarepr.py:102
- **Description:** eval()
- **Evidence (Line):** `eval(h + l)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1789. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/printing/tests/test_lambdarepr.py:115
- **Description:** eval()
- **Evidence (Line):** `eval(h + l)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1790. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/printing/tests/test_lambdarepr.py:128
- **Description:** eval()
- **Evidence (Line):** `eval(h + l)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1791. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/printing/tests/test_lambdarepr.py:141
- **Description:** eval()
- **Evidence (Line):** `eval(h + l)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1792. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/printing/tests/test_lambdarepr.py:153
- **Description:** eval()
- **Evidence (Line):** `eval(h + l)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1793. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/printing/tests/test_lambdarepr.py:158
- **Description:** eval()
- **Evidence (Line):** `# In each case, test eval() the lambdarepr() to make sure that`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1794. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/printing/tests/test_latex.py:3139
- **Description:** pickle load/dumps
- **Evidence (Line):** `assert pickle.loads(pickle.dumps(latex)) is latex`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1795. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/printing/tests/test_repr.py:30
- **Description:** eval()
- **Evidence (Line):** `# eval(srepr(expr)) == expr has to succeed in the right environment. The right`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1796. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/printing/tests/test_repr.py:41
- **Description:** eval()
- **Evidence (Line):** `the condition eval(srepr(expr))==expr holds.`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1797. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/printing/tests/test_repr.py:50
- **Description:** eval()
- **Evidence (Line):** `assert eval(string, ENV2) == expr`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1798. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/printing/tests/test_repr.py:72
- **Description:** eval()
- **Evidence (Line):** `assert eval(srepr(expr)) == expr`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1799. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/printing/tests/test_repr.py:184
- **Description:** eval()
- **Evidence (Line):** `assert eval(srepr(x), ENV) == x`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1800. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/printing/tests/test_repr.py:206
- **Description:** eval()
- **Evidence (Line):** `assert d == eval(srepr(d))`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1801. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/printing/tests/test_repr.py:33
- **Description:** exec()
- **Evidence (Line):** `exec("from sympy import *", ENV)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1802. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/printing/tests/test_repr.py:47
- **Description:** exec()
- **Evidence (Line):** `exec(import_stmt, ENV2)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1803. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/printing/tests/test_tensorflow.py:55
- **Description:** eval()
- **Evidence (Line):** `random_variables = [eval(tensorflow_code(i)) for i in random_matrices]`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1804. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/printing/tests/test_tensorflow.py:87
- **Description:** eval()
- **Evidence (Line):** `random_variables = [eval(tensorflow_code(i)) for i in random_matrices]`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1805. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/printing/tests/test_tensorflow.py:115
- **Description:** eval()
- **Evidence (Line):** `random_variables = [eval(tensorflow_code(i)) for i in random_matrices]`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1806. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/printing/tests/test_tensorflow.py:132
- **Description:** eval()
- **Evidence (Line):** `tf_rvs = [eval(tensorflow_code(i)) for i in rvs]`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1807. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/printing/tests/test_tensorflow.py:148
- **Description:** eval()
- **Evidence (Line):** `tf_rvs = [eval(tensorflow_code(i)) for i in rvs]`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1808. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/printing/tests/test_theanocode.py:636
- **Description:** eval()
- **Evidence (Line):** `np.testing.assert_allclose(float(e_a), e.eval())`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1809. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/printing/tests/test_theanocode.py:639
- **Description:** eval()
- **Evidence (Line):** `np.testing.assert_allclose(float(e_b), e.eval())`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1810. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/series/formal.py:963
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, p, x, n):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1811. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/series/tests/test_limits.py:169
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, arg):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1812. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/series/tests/test_order.py:298
- **Description:** eval()
- **Evidence (Line):** `def test_eval():`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1813. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/simplify/gammasimp.py:465
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, a, b):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1814. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/solvers/polysys.py:457
- **Description:** eval()
- **Evidence (Line):** `h = g.ltrim(var).eval(dict(mapping))`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1815. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/tensor/tensor.py:74
- **Description:** eval()
- **Evidence (Line):** `def deprecate_fun_eval():`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1816. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/tensor/tensor.py:77
- **Description:** eval()
- **Evidence (Line):** `The Tensor.fun_eval() method is deprecated. Use`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1817. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/tensor/tensor.py:2110
- **Description:** eval()
- **Evidence (Line):** `def fun_eval(self, *index_tuples):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1818. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/tensor/tensor.py:2111
- **Description:** eval()
- **Evidence (Line):** `deprecate_fun_eval()`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1819. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/testing/runtests.py:1243
- **Description:** exec()
- **Evidence (Line):** `exec(code, gl)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1820. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/testing/runtests.py:1487
- **Description:** exec()
- **Evidence (Line):** `#exec('from sympy import *') in test.globs`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1821. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/utilities/lambdify.py:163
- **Description:** eval()
- **Evidence (Line):** `module = eval(import_command)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1822. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/utilities/lambdify.py:170
- **Description:** exec()
- **Evidence (Line):** `exec(import_command, {}, namespace)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1823. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/utilities/lambdify.py:608
- **Description:** exec()
- **Evidence (Line):** `>>> exec('''`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1824. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/utilities/lambdify.py:620
- **Description:** exec()
- **Evidence (Line):** `>>> exec('''`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1825. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/utilities/lambdify.py:903
- **Description:** exec()
- **Evidence (Line):** `exec(ln, {}, namespace)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1826. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/utilities/lambdify.py:909
- **Description:** exec()
- **Evidence (Line):** `exec(ln, {}, namespace)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1827. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/utilities/lambdify.py:920
- **Description:** exec()
- **Evidence (Line):** `exec(c, namespace, funclocals)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1828. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/utilities/matchpy_connector.py:270
- **Description:** eval()
- **Evidence (Line):** `return eval(lambda_str, locals())`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1829. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/utilities/matchpy_connector.py:269
- **Description:** exec()
- **Evidence (Line):** `exec("from sympy import *")`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1830. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/utilities/tests/test_lambdify.py:862
- **Description:** eval()
- **Evidence (Line):** `assert func(a).eval(session=s) == 0.5`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1831. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/utilities/tests/test_lambdify.py:873
- **Description:** eval()
- **Evidence (Line):** `assert func(a).eval(session=s, feed_dict={a: 0}) == 0.5`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1832. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/utilities/tests/test_lambdify.py:885
- **Description:** eval()
- **Evidence (Line):** `assert func(a).eval(session=s, feed_dict={a: 0}) == 0.5`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1833. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/utilities/tests/test_lambdify.py:895
- **Description:** eval()
- **Evidence (Line):** `assert func(False, True).eval(session=s) == False`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1834. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/utilities/tests/test_lambdify.py:905
- **Description:** eval()
- **Evidence (Line):** `assert func(-1).eval(session=s) == -1`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1835. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/utilities/tests/test_lambdify.py:906
- **Description:** eval()
- **Evidence (Line):** `assert func(0).eval(session=s) == 0`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1836. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/utilities/tests/test_lambdify.py:907
- **Description:** eval()
- **Evidence (Line):** `assert func(1).eval(session=s) == 1`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1837. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/utilities/tests/test_lambdify.py:917
- **Description:** eval()
- **Evidence (Line):** `assert func(-2).eval(session=s) == 4`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1838. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/utilities/tests/test_lambdify.py:927
- **Description:** eval()
- **Evidence (Line):** `assert func(-2).eval(session=s) == -2`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1839. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/utilities/tests/test_lambdify.py:937
- **Description:** eval()
- **Evidence (Line):** `assert func(1).eval(session=s) == True`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1840. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/utilities/tests/test_lambdify.py:954
- **Description:** eval()
- **Evidence (Line):** `assert func1(a).eval(session=s) == 1`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1841. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/utilities/tests/test_lambdify.py:955
- **Description:** eval()
- **Evidence (Line):** `assert func2(a).eval(session=s) == 2`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1842. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/utilities/tests/test_lambdify.py:957
- **Description:** eval()
- **Evidence (Line):** `tensorflow_result = func3(a).eval(session=s)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1843. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/utilities/tests/test_lambdify.py:961
- **Description:** eval()
- **Evidence (Line):** `tensorflow_result = func4(a).eval(session=s)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1844. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/utilities/tests/test_lambdify.py:975
- **Description:** eval()
- **Evidence (Line):** `assert fcall.eval(session=s) == 5.0`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1845. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/utilities/tests/test_lambdify.py:1521
- **Description:** eval()
- **Evidence (Line):** `assert eval(lambdastr((f, fx), f/fx))(10, 5) == 2`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1846. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/utilities/tests/test_lambdify.py:1523
- **Description:** eval()
- **Evidence (Line):** `eval(lambdastr((f, fx), f/fx, dummify=False)))`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1847. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/utilities/tests/test_lambdify.py:1524
- **Description:** eval()
- **Evidence (Line):** `assert eval(lambdastr((f, fx), f/fx, dummify=True))(10, 5) == 2`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1848. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/utilities/tests/test_lambdify.py:1525
- **Description:** eval()
- **Evidence (Line):** `assert eval(lambdastr((fx, f), f/fx, dummify=True))(S(10), 5) == S.Half`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1849. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/utilities/tests/test_lambdify.py:1527
- **Description:** eval()
- **Evidence (Line):** `assert eval(lambdastr(fx, 1 + fx, dummify=True))(41) == 42`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1850. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/utilities/tests/test_matchpy_connector.py:151
- **Description:** pickle load/dumps
- **Evidence (Line):** `a2 = pickle.loads(pickle.dumps(a1))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1851. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/utilities/tests/test_matchpy_connector.py:155
- **Description:** pickle load/dumps
- **Evidence (Line):** `a2 = pickle.loads(pickle.dumps(a1))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1852. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/utilities/tests/test_matchpy_connector.py:159
- **Description:** pickle load/dumps
- **Evidence (Line):** `a2 = pickle.loads(pickle.dumps(a1))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1853. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/utilities/tests/test_matchpy_connector.py:163
- **Description:** pickle load/dumps
- **Evidence (Line):** `a2 = pickle.loads(pickle.dumps(a1))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1854. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/utilities/tests/test_pickling.py:74
- **Description:** pickle load/dumps
- **Evidence (Line):** `b = pickle.loads(pickle.dumps(a, protocol))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1855. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/utilities/tests/test_pickling.py:197
- **Description:** pickle load/dumps
- **Evidence (Line):** `copiers += [lambda x: pickle.loads(pickle.dumps(x, proto))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1856. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/utilities/tests/test_pickling.py:200
- **Description:** pickle load/dumps
- **Evidence (Line):** `copiers += [lambda x: cloudpickle.loads(cloudpickle.dumps(x))]`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1857. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/utilities/tests/test_pickling.py:712
- **Description:** pickle load/dumps
- **Evidence (Line):** `assert pickle.loads(pickle.dumps(S.Half)) == S.Half`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1858. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/sympy/utilities/tests/test_pickling.py:723
- **Description:** pickle load/dumps
- **Evidence (Line):** `assert pickle.loads(data) == sqrt(2)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1859. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/threadpoolctl.py:1708
- **Description:** exec()
- **Evidence (Line):** `exec(options.command)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1860. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_dynamo/aot_compile.py:507
- **Description:** pickle load/dumps
- **Evidence (Line):** `results: list[bytes] = pickle.loads(data)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1861. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_dynamo/aot_compile_types.py:120
- **Description:** pickle load/dumps
- **Evidence (Line):** `state = pickle.loads(data)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1862. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_dynamo/aot_compile_types.py:191
- **Description:** pickle load/dumps
- **Evidence (Line):** `bundle = pickle.loads(data)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1863. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_dynamo/backends/torchxla.py:17
- **Description:** eval()
- **Evidence (Line):** `def openxla_eval(`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1864. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_dynamo/bytecode_debugger.py:412
- **Description:** eval()
- **Evidence (Line):** `def _get_stack_for_eval(`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1865. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_dynamo/bytecode_debugger.py:430
- **Description:** eval()
- **Evidence (Line):** `eval_locals["__stack__"] = self._get_stack_for_eval(state, is_current_frame)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1866. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_dynamo/bytecode_debugger.py:684
- **Description:** eval()
- **Evidence (Line):** `result = eval(arg, frame_globals, eval_locals)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1867. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_dynamo/bytecode_debugger.py:709
- **Description:** eval()
- **Evidence (Line):** `result = eval(cmd, frame_globals, eval_locals)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1868. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_dynamo/bytecode_debugger.py:717
- **Description:** exec()
- **Evidence (Line):** `exec(cmd, frame_globals, eval_locals)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1869. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_dynamo/convert_frame.py:693
- **Description:** exec()
- **Evidence (Line):** `return ConvertFrameReturn(skip_reason="frame was generated by exec()")`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1870. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_dynamo/convert_frame.py:1145
- **Description:** exec()
- **Evidence (Line):** `# exec() should be fine because we only define a function in the scope`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1871. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_dynamo/convert_frame.py:1148
- **Description:** exec()
- **Evidence (Line):** `exec(func_source, namespace)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1872. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_dynamo/functional_export.py:881
- **Description:** exec()
- **Evidence (Line):** `exec(func_str, {"op": fn}, out)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1873. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_dynamo/guards.py:5337
- **Description:** eval()
- **Evidence (Line):** `tensor_id = id(eval(tensor_source, global_scope, scope))`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1874. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_dynamo/guards.py:5513
- **Description:** eval()
- **Evidence (Line):** `fail_reason = eval(part, global_scope, scope)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1875. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_dynamo/guards.py:5677
- **Description:** eval()
- **Evidence (Line):** `eval(guard, guard_manager.global_scope, local_scope)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1876. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_dynamo/guards.py:2174
- **Description:** exec()
- **Evidence (Line):** `exec(pycode, globals_for_guard_fn, out)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1877. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_dynamo/package.py:267
- **Description:** eval()
- **Evidence (Line):** `fn = attr[ast.literal_eval(part[index_begin + 1 : -1])]`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1878. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_dynamo/package.py:135
- **Description:** pickle load/dumps
- **Evidence (Line):** `return pickle.loads(guards_state)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1879. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_dynamo/package.py:1237
- **Description:** pickle load/dumps
- **Evidence (Line):** `entry = pickle.loads(pickled_content)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1880. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_dynamo/pgo.py:827
- **Description:** pickle load/dumps
- **Evidence (Line):** `_CODE_STATE = pickle.loads(content)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1881. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_dynamo/pgo.py:868
- **Description:** pickle load/dumps
- **Evidence (Line):** `code_state = pickle.loads(payload)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1882. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_dynamo/precompile_context.py:40
- **Description:** pickle load/dumps
- **Evidence (Line):** `result = pickle.loads(self.content)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1883. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_dynamo/repro/after_aot.py:1163
- **Description:** eval()
- **Evidence (Line):** `syms[f"_derived_{idx}"] = eval(code, {"__builtins__": {}}, syms)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1884. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_dynamo/repro/after_dynamo.py:376
- **Description:** eval()
- **Evidence (Line):** `gm.eval()`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1885. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_dynamo/repro/after_dynamo.py:502
- **Description:** eval()
- **Evidence (Line):** `mod.eval()`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1886. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_dynamo/repro/after_dynamo.py:503
- **Description:** eval()
- **Evidence (Line):** `opt_mod.eval()  # type: ignore[union-attr]`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1887. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_dynamo/test_minifier_common.py:164
- **Description:** exec()
- **Evidence (Line):** `exec(code, {"__name__": "__main__", "__compile_source__": code})`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1888. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_dynamo/utils.py:2901
- **Description:** eval()
- **Evidence (Line):** `fn = eval(f"lambda {','.join(vars)}: ({','.join(rotated)})")`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1889. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_dynamo/utils.py:3212
- **Description:** eval()
- **Evidence (Line):** `args = ast.literal_eval(f"({count_repr[6:-1]},)")`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1890. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_dynamo/variables/builtin.py:1507
- **Description:** eval()
- **Evidence (Line):** `result = ast.literal_eval(tree)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1891. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_dynamo/variables/builtin.py:1512
- **Description:** eval()
- **Evidence (Line):** `result = eval(`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1892. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_dynamo/variables/builtin.py:1521
- **Description:** eval()
- **Evidence (Line):** `def call_eval(`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1893. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_dynamo/variables/tensor.py:497
- **Description:** eval()
- **Evidence (Line):** `# eval("super(L['mod'].model.model.encoder.embed_positions.forward__class__,`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1894. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_dynamo/variables/tensor.py:499
- **Description:** eval()
- **Evidence (Line):** `# Which is incorrect, and violates the invariant that all sources should be eval()-able against the `
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1895. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_dynamo/variables/tensor.py:500
- **Description:** eval()
- **Evidence (Line):** `_input_associated_real_value = eval(self.source.name, scope)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1896. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_dynamo/variables/user_defined.py:4227
- **Description:** exec()
- **Evidence (Line):** `# synthesizes the generated one via exec (co_filename "<string>"), so a`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1897. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_export/serde/serialize.py:2938
- **Description:** exec()
- **Evidence (Line):** `# e.g.., `exec('from torch.utils._sympy.functions import *', ...)``
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1898. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_functorch/_aot_autograd/autograd_cache.py:1096
- **Description:** pickle load/dumps
- **Evidence (Line):** `compiled_fn, lambda: pickle.loads(pickled_content)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1899. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_functorch/_aot_autograd/codegen.py:134
- **Description:** exec()
- **Evidence (Line):** `NOT contain the interpreter ``__builtins__`` that exec() would otherwise inject,`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1900. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_functorch/_aot_autograd/codegen.py:216
- **Description:** exec()
- **Evidence (Line):** `# exec() mutates ``globals_dict`` in place, injecting ``__builtins__`` (and any`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1901. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_functorch/_aot_autograd/codegen.py:223
- **Description:** exec()
- **Evidence (Line):** `exec(code, globals_dict, local_dict)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1902. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_functorch/compilers.py:112
- **Description:** eval()
- **Evidence (Line):** `f = torch.jit.freeze(f.eval())`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1903. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_functorch/compilers.py:234
- **Description:** eval()
- **Evidence (Line):** `f = torch.jit.freeze(f.eval())`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1904. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_functorch/compilers.py:358
- **Description:** pickle load/dumps
- **Evidence (Line):** `inputs_meta = pickle.load(f)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1905. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_guards.py:1446
- **Description:** eval()
- **Evidence (Line):** `value = eval(self._name_template, globals, locals)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1906. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_guards.py:1510
- **Description:** eval()
- **Evidence (Line):** `value = eval(self._name_template.format(tmpvar), globals, locals)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1907. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_higher_order_ops/invoke_leaf_function.py:154
- **Description:** eval()
- **Evidence (Line):** `result = eval(expr, {"__builtins__": {}}, namespace)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1908. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_higher_order_ops/triton_kernel_wrap.py:1436
- **Description:** exec()
- **Evidence (Line):** `exec(code, namespace)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1909. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_inductor/autotune_process.py:175
- **Description:** pickle load/dumps
- **Evidence (Line):** `return pickle.load(read_pipe)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1910. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_inductor/cache.py:146
- **Description:** eval()
- **Evidence (Line):** `key_bytes = literal_eval(key_bytes_repr)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1911. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_inductor/cache.py:153
- **Description:** eval()
- **Evidence (Line):** `value_bytes = literal_eval(value_bytes_repr)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1912. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_inductor/cache.py:160
- **Description:** pickle load/dumps
- **Evidence (Line):** `key = pickle.loads(key_bytes)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1913. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_inductor/cache.py:166
- **Description:** pickle load/dumps
- **Evidence (Line):** `value = pickle.loads(value_bytes)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1914. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_inductor/cache.py:202
- **Description:** pickle load/dumps
- **Evidence (Line):** `cache._cache = pickle.load(fp)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1915. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_inductor/cache.py:356
- **Description:** pickle load/dumps
- **Evidence (Line):** `value = pickle.loads(value_bytes)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1916. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_inductor/codecache.py:3818
- **Description:** exec()
- **Evidence (Line):** `"is mounted with noexec (e.g., by default Docker mounts tmp file systems "`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1917. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_inductor/codecache.py:1858
- **Description:** pickle load/dumps
- **Evidence (Line):** `yield pickle.loads(content), content, True`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1918. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_inductor/codecache.py:1878
- **Description:** pickle load/dumps
- **Evidence (Line):** `yield pickle.loads(content), content, False`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1919. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_inductor/codegen/cpp_wrapper_cpu.py:1785
- **Description:** exec()
- **Evidence (Line):** `# as a global variable passed when calling exec(code, mod.__dict__, mod.__dict__).`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1920. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_inductor/codegen/wrapper.py:2674
- **Description:** exec()
- **Evidence (Line):** `exec(tuning_code, scope)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1921. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_inductor/codegen/wrapper.py:3236
- **Description:** pickle load/dumps
- **Evidence (Line):** `output.writeline(f"{name} = pickle.loads({pickle.dumps(value)!r})")`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1922. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_inductor/comm_analysis.py:472
- **Description:** eval()
- **Evidence (Line):** `fn = eval(py_kernel_name)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1923. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_inductor/compile_worker/subproc_pool.py:147
- **Description:** pickle load/dumps
- **Evidence (Line):** `return pickle.loads(data)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1924. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_inductor/compiler_bisector.py:355
- **Description:** eval()
- **Evidence (Line):** `config_data[key] = eval(value)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1925. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_inductor/debug.py:1460
- **Description:** pickle load/dumps
- **Evidence (Line):** `args, kwargs = pickle.load(f)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1926. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_inductor/fuzzer.py:715
- **Description:** pickle load/dumps
- **Evidence (Line):** `state = pickle.load(f)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1927. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_inductor/fx_passes/efficient_conv_bn_eval.py:24
- **Description:** eval()
- **Evidence (Line):** `def efficient_conv_bn_eval(`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1928. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_inductor/fx_passes/pre_grad.py:602
- **Description:** eval()
- **Evidence (Line):** `fused_conv = fuse_conv_bn_eval(conv, bn)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1929. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_inductor/kernel/flex/flex_flash_attention.py:208
- **Description:** eval()
- **Evidence (Line):** ``eval()` returns None to keep the node inert (no simplification/flattening).`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1930. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_inductor/kernel/flex/flex_flash_attention.py:216
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, *args):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1931. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_inductor/kernel/vendored_templates/cutedsl/wrappers/dense_blockscaled_gemm_kernel.py:280
- **Description:** exec()
- **Evidence (Line):** `exec(epilogue_op, scope)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1932. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_inductor/ops_handler.py:859
- **Description:** exec()
- **Evidence (Line):** `exec(code.getvalue(), ctx)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1933. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_inductor/profiler.py:219
- **Description:** eval()
- **Evidence (Line):** `parsed_key = ast.literal_eval(key)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1934. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_inductor/runtime/caching/interfaces.py:1261
- **Description:** pickle load/dumps
- **Evidence (Line):** `cache_entry = cast(CacheEntry, pickle.loads(pickled_value))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1935. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_inductor/runtime/compile_tasks.py:37
- **Description:** exec()
- **Evidence (Line):** `exec(code, mod.__dict__, mod.__dict__)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1936. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_inductor/runtime/triton_heuristics.py:2835
- **Description:** exec()
- **Evidence (Line):** `exec(launcher_code, scope)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1937. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_inductor/runtime/triton_heuristics.py:5169
- **Description:** exec()
- **Evidence (Line):** `exec(line, scope)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1938. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_inductor/runtime/triton_heuristics.py:5170
- **Description:** exec()
- **Evidence (Line):** `exec(f"grid_0 = {self.x_grid}", scope)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1939. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_inductor/runtime/triton_heuristics.py:5171
- **Description:** exec()
- **Evidence (Line):** `exec(f"grid_1 = {self.y_grid}", scope)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1940. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_inductor/runtime/triton_heuristics.py:5172
- **Description:** exec()
- **Evidence (Line):** `exec(f"grid_2 = {self.z_grid}", scope)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1941. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_inductor/runtime/triton_lazy_compile.py:122
- **Description:** eval()
- **Evidence (Line):** `kernel_obj = eval(kernel_source.strip())`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1942. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_inductor/select_algorithm.py:3447
- **Description:** eval()
- **Evidence (Line):** `tile_vals = eval(tile)  # type: ignore[arg-type]`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1943. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_inductor/select_algorithm.py:5659
- **Description:** eval()
- **Evidence (Line):** `tile_vals = eval(tile)  # type: ignore[arg-type]`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1944. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_inductor/standalone_compile.py:687
- **Description:** exec()
- **Evidence (Line):** `exec(compile(python_code, "<compile_to_python>", "exec"), namespace)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1945. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_inductor/standalone_compile.py:243
- **Description:** pickle load/dumps
- **Evidence (Line):** `graph = pickle.load(f)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1946. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_inductor/utils.py:220
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, value: sympy.Expr) -> sympy.Expr | None:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1947. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_inductor/utils.py:819
- **Description:** exec()
- **Evidence (Line):** `exec(`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1948. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_inductor/utils.py:864
- **Description:** exec()
- **Evidence (Line):** `exec(`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1949. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_library/infer_schema.py:94
- **Description:** eval()
- **Evidence (Line):** `return eval(annotation_type, pf_globals, pf_locals)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1950. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_precompile.py:1122
- **Description:** eval()
- **Evidence (Line):** `found[target.id] = ast.literal_eval(node.value)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1951. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_precompile.py:967
- **Description:** exec()
- **Evidence (Line):** `#     exec(open("this_file.py").read(), ns)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1952. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_precompile.py:1174
- **Description:** exec()
- **Evidence (Line):** `#     exec(open("this_file.py").read(), ns)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1953. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_precompile.py:1570
- **Description:** exec()
- **Evidence (Line):** `exec(compile(python_code, "<precompile>", "exec"), module_ns)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1954. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_utils.py:1406
- **Description:** pickle load/dumps
- **Evidence (Line):** `snapshot_dict = cast(_Snapshot, pickle.load(f))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1955. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_vendor/quack/_compile_worker.py:135
- **Description:** pickle load/dumps
- **Evidence (Line):** `return pickle.loads(data)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1956. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_vendor/quack/autotuner.py:278
- **Description:** pickle load/dumps
- **Evidence (Line):** `return pickle.loads(body)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1957. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/amp/autocast_mode.py:144
- **Description:** eval()
- **Evidence (Line):** `model = Net().eval()`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1958. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/amp/autocast_mode.py:164
- **Description:** eval()
- **Evidence (Line):** `model = TestModel(input_size, num_classes).eval()`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1959. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/ao/nn/quantizable/modules/activation.py:193
- **Description:** eval()
- **Evidence (Line):** `observed.eval()`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1960. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/ao/nn/quantizable/modules/rnn.py:606
- **Description:** eval()
- **Evidence (Line):** `observed.eval()`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1961. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/ao/ns/_numeric_suite_fx.py:11
- **Description:** eval()
- **Evidence (Line):** `m = torch.nn.Sequential(torch.nn.Conv2d(1, 1, 1)).eval()`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1962. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/ao/pruning/_experimental/data_sparsifier/lightning/callbacks/data_sparsity.py:54
- **Description:** eval()
- **Evidence (Line):** `self.sparsified = deepcopy(pl_module.model).eval()`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1963. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/ao/quantization/backend_config/onednn.py:106
- **Description:** eval()
- **Evidence (Line):** `fused_linear = nn.utils.fusion.fuse_linear_bn_eval(linear, bn)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1964. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/ao/quantization/backend_config/onednn.py:164
- **Description:** eval()
- **Evidence (Line):** `fused_conv = nn.utils.fusion.fuse_conv_bn_eval(conv, bn)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1965. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/ao/quantization/backend_config/onednn.py:247
- **Description:** eval()
- **Evidence (Line):** `fused_conv = nn.utils.fusion.fuse_conv_bn_eval(conv, bn)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1966. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/ao/quantization/backend_config/onednn.py:348
- **Description:** eval()
- **Evidence (Line):** `fused_conv = nn.utils.fusion.fuse_conv_bn_eval(conv, bn)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1967. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/ao/quantization/backend_config/onednn.py:441
- **Description:** eval()
- **Evidence (Line):** `fused_conv = nn.utils.fusion.fuse_conv_bn_eval(conv, bn)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1968. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/ao/quantization/fuse_modules.py:177
- **Description:** eval()
- **Evidence (Line):** `>>> m = M().eval()`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1969. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/ao/quantization/fuse_modules.py:183
- **Description:** eval()
- **Evidence (Line):** `>>> m = M().eval()`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1970. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/ao/quantization/fuser_method_mappings.py:68
- **Description:** eval()
- **Evidence (Line):** `return nn.utils.fuse_conv_bn_eval(conv, bn)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1971. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/ao/quantization/fuser_method_mappings.py:126
- **Description:** eval()
- **Evidence (Line):** `fused_conv = nn.utils.fusion.fuse_conv_bn_eval(conv, bn)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1972. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/ao/quantization/fuser_method_mappings.py:169
- **Description:** eval()
- **Evidence (Line):** `return nn.utils.fusion.fuse_linear_bn_eval(linear, bn)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1973. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/ao/quantization/fuser_method_mappings.py:198
- **Description:** eval()
- **Evidence (Line):** `return nn.utils.fusion.fuse_conv_bn_eval(convt, bn, transpose=True)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1974. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/ao/quantization/quantize.py:476
- **Description:** eval()
- **Evidence (Line):** `model.eval()`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1975. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/ao/quantization/quantize.py:568
- **Description:** eval()
- **Evidence (Line):** `model.eval()`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1976. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/ao/quantization/quantize_fx.py:221
- **Description:** eval()
- **Evidence (Line):** `m = Model().eval()`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1977. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/ao/quantization/quantize_fx.py:313
- **Description:** eval()
- **Evidence (Line):** `float_model = M().eval()`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1978. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/ao/quantization/quantize_fx.py:317
- **Description:** eval()
- **Evidence (Line):** `model.eval()`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1979. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/ao/quantization/quantize_jit.py:131
- **Description:** eval()
- **Evidence (Line):** `model.eval()`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1980. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/ao/quantization/quantize_jit.py:286
- **Description:** eval()
- **Evidence (Line):** `float_model.eval()`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1981. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/ao/quantization/quantize_jit.py:292
- **Description:** eval()
- **Evidence (Line):** `model.eval()`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1982. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/ao/quantization/quantize_jit.py:339
- **Description:** eval()
- **Evidence (Line):** `float_model.eval()`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1983. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/ao/quantization/quantize_jit.py:345
- **Description:** eval()
- **Evidence (Line):** `model.eval()`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1984. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/ao/quantization/quantize_jit.py:413
- **Description:** eval()
- **Evidence (Line):** `float_model.eval()`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1985. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/autograd/grad_mode.py:245
- **Description:** eval()
- **Evidence (Line):** ``model.eval()` in addition to using this context manager.`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1986. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/cuda/_annotate_cuda_graph_trace.py:328
- **Description:** pickle load/dumps
- **Evidence (Line):** `annotations = pickle.load(f)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1987. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/cuda/_graph_annotations.py:842
- **Description:** exec()
- **Evidence (Line):** `torch_cuda_graph.raw_cuda_graph_exec()`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1988. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/cuda/_memory_viz.py:774
- **Description:** pickle load/dumps
- **Evidence (Line):** `data = pickle.load(sys.stdin.buffer)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1989. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/cuda/_memory_viz.py:777
- **Description:** pickle load/dumps
- **Evidence (Line):** `data = pickle.load(f)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1990. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/cuda/graphs.py:789
- **Description:** exec()
- **Evidence (Line):** `def raw_cuda_graph_exec(self) -> int:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1991. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/cuda/graphs.py:790
- **Description:** exec()
- **Evidence (Line):** `r"""Returns the underlying cudaGraphExec_t. ``instantiate`` must have been called if ``keep_graph`` `
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1992. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/cuda/graphs.py:794
- **Description:** exec()
- **Evidence (Line):** `return super().raw_cuda_graph_exec()`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1993. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/cuda/graphs.py:985
- **Description:** exec()
- **Evidence (Line):** `init_value=self.raw_cuda_graph_exec()`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1994. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/distributed/_tools/memory_tracker.py:223
- **Description:** pickle load/dumps
- **Evidence (Line):** `stats = pickle.load(f)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1995. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/distributed/checkpoint/_pg_transport.py:301
- **Description:** pickle load/dumps
- **Evidence (Line):** `meta: _StateDictMeta = pickle.loads(buf.cpu().numpy().tobytes())`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1996. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/distributed/checkpoint/filesystem.py:938
- **Description:** pickle load/dumps
- **Evidence (Line):** `metadata = pickle.load(metadata_file)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1997. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/distributed/elastic/multiprocessing/redirects.py:202
- **Description:** os.system()
- **Evidence (Line):** `os.system("echo system stdouts are also redirected")`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 1998. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/distributed/elastic/rendezvous/dynamic_rendezvous.py:444
- **Description:** pickle load/dumps
- **Evidence (Line):** `self._state = pickle.loads(state_bits)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 1999. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/distributed/flight_recorder/components/builder.py:111
- **Description:** eval()
- **Evidence (Line):** `ranks = ast.literal_eval(pg_config[global_rank][pg_uid]["ranks"])`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2000. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/distributed/flight_recorder/components/builder.py:119
- **Description:** eval()
- **Evidence (Line):** `ranks = ast.literal_eval(ranks)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2001. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/distributed/flight_recorder/components/builder.py:401
- **Description:** eval()
- **Evidence (Line):** `ranks = ast.literal_eval(pg_config["ranks"])`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2002. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/distributed/flight_recorder/components/loader.py:37
- **Description:** pickle load/dumps
- **Evidence (Line):** `dump = pickle.load(infile)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 2003. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/distributed/fsdp/_flat_param.py:88
- **Description:** eval()
- **Evidence (Line):** `# Env var toggling whether when model is in .eval() mode, should we run in fp32`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2004. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/distributed/fsdp/_runtime_utils.py:536
- **Description:** eval()
- **Evidence (Line):** `# exiting eval() mode.`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2005. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/distributed/fsdp/_runtime_utils.py:761
- **Description:** eval()
- **Evidence (Line):** `# (i.e. model.eval() + full precision in eval was configured), don't downcast gradient.`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2006. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/distributed/nn/api/remote_module.py:442
- **Description:** eval()
- **Evidence (Line):** `def eval(self) -> Self:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2007. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/distributed/nn/api/remote_module.py:443
- **Description:** eval()
- **Evidence (Line):** `return self.module_rref.rpc_sync().eval()  # type: ignore[operator, union-attr]`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2008. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/distributed/nn/jit/instantiator.py:94
- **Description:** exec()
- **Evidence (Line):** `exec(code, module.__dict__)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2009. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/distributed/pipelining/schedules.py:560
- **Description:** eval()
- **Evidence (Line):** `def eval(`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2010. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/distributed/pipelining/schedules.py:611
- **Description:** eval()
- **Evidence (Line):** `>>> output = schedule.eval(x, mask=mask)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2011. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/distributed/pipelining/schedules.py:614
- **Description:** eval()
- **Evidence (Line):** `>>> output = schedule.eval(`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2012. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/distributed/pipelining/schedules.py:977
- **Description:** eval()
- **Evidence (Line):** `"Please call eval() instead."`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2013. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/distributed/pipelining/schedules.py:2222
- **Description:** eval()
- **Evidence (Line):** `"Please call eval() instead."`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2014. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/export/_unlift.py:221
- **Description:** exec()
- **Evidence (Line):** `exec(code_str, namespace)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2015. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/export/exported_program.py:1497
- **Description:** eval()
- **Evidence (Line):** `def _eval(self, mode: bool = True):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2016. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/export/exported_program.py:1498
- **Description:** eval()
- **Evidence (Line):** `raise NotImplementedError("Calling eval() is not supported yet.")`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2017. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/export/pt2_archive/_package.py:980
- **Description:** pickle load/dumps
- **Evidence (Line):** `constants[constant_fqn] = pickle.loads(constant_bytes)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 2018. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/fx/experimental/optimization.py:109
- **Description:** eval()
- **Evidence (Line):** `fused_layer = fuse_conv_bn_eval(first_layer, bn)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2019. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/fx/experimental/optimization.py:111
- **Description:** eval()
- **Evidence (Line):** `fused_layer = fuse_linear_bn_eval(first_layer, bn)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2020. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/fx/experimental/proxy_tensor.py:170
- **Description:** eval()
- **Evidence (Line):** `return eval(f"lambda {argnames}: fn({argnames})", {"fn": fn})`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2021. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/fx/experimental/rewriter.py:47
- **Description:** exec()
- **Evidence (Line):** `exec(code, globals_dict)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2022. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/fx/experimental/symbolic_shapes.py:7045
- **Description:** eval()
- **Evidence (Line):** `return eval(code, SYMPY_INTERP, args)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2023. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/fx/experimental/symbolic_shapes.py:7055
- **Description:** eval()
- **Evidence (Line):** `return eval(code, SYMPY_INTERP, args)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2024. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/fx/experimental/symbolic_shapes.py:7063
- **Description:** eval()
- **Evidence (Line):** `return eval(code, SYMPY_INTERP, {"L": dict(zip(arg_names, args))})`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2025. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/fx/experimental/symbolic_shapes.py:8971
- **Description:** eval()
- **Evidence (Line):** `def _should_skip_static_eval(self, expr: SympyBoolean) -> bool:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2026. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/fx/experimental/symbolic_shapes.py:9018
- **Description:** eval()
- **Evidence (Line):** `if self._should_skip_static_eval(expr):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2027. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/fx/graph.py:636
- **Description:** eval()
- **Evidence (Line):** `# signed zero components, e.g. eval("(-0-1j)") loses the sign.`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2028. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/fx/graph_module.py:145
- **Description:** exec()
- **Evidence (Line):** `exec(compile(src, key, "exec", dont_inherit=True), globals)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2029. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/fx/operator_schemas.py:95
- **Description:** eval()
- **Evidence (Line):** `return eval(ts_type.annotation_str, _type_eval_globals)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2030. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/fx/passes/splitter_base.py:833
- **Description:** eval()
- **Evidence (Line):** `split_mod.eval()`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2031. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/jit/_freeze.py:59
- **Description:** eval()
- **Evidence (Line):** `scripted_module = torch.jit.script(MyModule(2, 3).eval())`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2032. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/jit/_freeze.py:80
- **Description:** eval()
- **Evidence (Line):** `scripted_module = torch.jit.script(MyModule2().eval())`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2033. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/jit/_freeze.py:119
- **Description:** eval()
- **Evidence (Line):** `"Please call .eval() on your module before freezing."`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2034. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/jit/_freeze.py:173
- **Description:** eval()
- **Evidence (Line):** `frozen_mod = torch.jit.freeze(torch.jit.script(mod.eval()), optimize=False)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2035. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/jit/_freeze.py:225
- **Description:** eval()
- **Evidence (Line):** `frozen_mod = torch.jit.optimize_for_inference(torch.jit.script(mod.eval()))`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2036. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/jit/_freeze.py:244
- **Description:** eval()
- **Evidence (Line):** `mod = freeze(mod.eval(), preserved_attrs=other_methods)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2037. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/jit/_trace.py:504
- **Description:** eval()
- **Evidence (Line):** `"Did you forget to call .eval() on your model? Nodes:\n"`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2038. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/jit/annotations.py:210
- **Description:** eval()
- **Evidence (Line):** `return eval(bytecode, glob, loc)  # type: ignore[arg-type] # noqa: P204`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2039. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/jit/frontend.py:595
- **Description:** exec()
- **Evidence (Line):** `exec(ignore_func_str, g)  # noqa: P204`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2040. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/jit/unsupported_tensor_ops.py:9
- **Description:** exec()
- **Evidence (Line):** `exec(code, glob, loc)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2041. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/multiprocessing/queue.py:21
- **Description:** pickle load/dumps
- **Evidence (Line):** `return pickle.loads(buf)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 2042. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/multiprocessing/spawn.py:208
- **Description:** pickle load/dumps
- **Evidence (Line):** `original_trace = pickle.load(fh)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 2043. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/nn/modules/activation.py:1119
- **Description:** eval()
- **Evidence (Line):** `- training is disabled (using ``.eval()``)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2044. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/nn/modules/batchnorm.py:724
- **Description:** eval()
- **Evidence (Line):** `synchronization is disabled when ``model.eval()`` is set or if`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2045. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/nn/modules/batchnorm.py:836
- **Description:** eval()
- **Evidence (Line):** `# Don't sync batchnorm stats in inference mode (model.eval()).`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2046. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/nn/modules/module.py:2916
- **Description:** eval()
- **Evidence (Line):** `def eval(self) -> Self:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2047. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/nn/modules/module.py:2927
- **Description:** eval()
- **Evidence (Line):** ``.eval()` and several similar mechanisms that may be confused with it.`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2048. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/nn/modules/transformer.py:722
- **Description:** eval()
- **Evidence (Line):** `- training is disabled (using ``.eval()``)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2049. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/nn/utils/fusion.py:20
- **Description:** eval()
- **Evidence (Line):** `def fuse_conv_bn_eval(`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2050. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/nn/utils/fusion.py:111
- **Description:** eval()
- **Evidence (Line):** `def fuse_linear_bn_eval(`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2051. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/onnx/_internal/exporter/_building.py:611
- **Description:** eval()
- **Evidence (Line):** `def eval(`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2052. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/onnx/_internal/exporter/_compat.py:80
- **Description:** eval()
- **Evidence (Line):** `"Calling model.eval() before export is recommended.",`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2053. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/onnx/_internal/torchscript_exporter/verification.py:483
- **Description:** eval()
- **Evidence (Line):** `model.eval()`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2054. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/optim/lbfgs.py:543
- **Description:** eval()
- **Evidence (Line):** `max_eval (int, optional): maximal number of function evaluations per optimization`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2055. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/package/package_importer.py:412
- **Description:** exec()
- **Evidence (Line):** `exec(code, ns)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2056. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/testing/_internal/common_fsdp.py:309
- **Description:** eval()
- **Evidence (Line):** `self.eval()`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2057. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/testing/_internal/common_modules.py:3044
- **Description:** eval()
- **Evidence (Line):** `# since the fast path requires no_grad mode, we run the fast path in .eval()`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2058. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/testing/_internal/common_quantization.py:915
- **Description:** eval()
- **Evidence (Line):** `module = module.eval()`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2059. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/testing/_internal/common_quantization.py:918
- **Description:** eval()
- **Evidence (Line):** `model = get_script_module(module, tracing, inputs[0]).eval()`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2060. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/testing/_internal/common_quantization.py:1271
- **Description:** eval()
- **Evidence (Line):** `model.eval()`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2061. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/testing/_internal/common_quantization.py:1275
- **Description:** eval()
- **Evidence (Line):** `model.eval()`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2062. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/testing/_internal/common_quantization.py:3354
- **Description:** eval()
- **Evidence (Line):** `return Model(N, K, bias, example_input, epilogue, output_dtype).eval()`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2063. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/testing/_internal/common_quantization.py:3486
- **Description:** eval()
- **Evidence (Line):** `).eval()`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2064. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/testing/_internal/distributed/_shard/sharded_tensor/_test_ops_common.py:139
- **Description:** exec()
- **Evidence (Line):** `builtins.exec(code_str, g)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2065. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/testing/_internal/distributed/distributed_test.py:9257
- **Description:** eval()
- **Evidence (Line):** `test_model.eval()`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2066. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/testing/_internal/distributed/distributed_test.py:9258
- **Description:** eval()
- **Evidence (Line):** `test_local_model.eval()`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2067. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/testing/_internal/distributed/distributed_test.py:9276
- **Description:** eval()
- **Evidence (Line):** `def test_ddp_sync_bn_training_vs_eval(self):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2068. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/testing/_internal/distributed/distributed_test.py:9305
- **Description:** eval()
- **Evidence (Line):** `model_inference.eval()`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2069. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/testing/_internal/distributed/distributed_test.py:9313
- **Description:** eval()
- **Evidence (Line):** `# Ensure sync does not occur in eval() mode.`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2070. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/testing/_internal/distributed/nn/api/remote_module_test.py:267
- **Description:** eval()
- **Evidence (Line):** `def test_train_eval(self):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2071. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/testing/_internal/distributed/nn/api/remote_module_test.py:283
- **Description:** eval()
- **Evidence (Line):** `remote_module.eval()`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2072. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/testing/_internal/distributed/rpc/dist_autograd_test.py:324
- **Description:** exec()
- **Evidence (Line):** `self._verify_graph_for_rpc_call_exec(next(iter(send_functions.values())))`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2073. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/testing/_internal/distributed/rpc/dist_autograd_test.py:414
- **Description:** exec()
- **Evidence (Line):** `self._verify_graph_for_rpc_call_exec(next(iter(send_functions.values())))`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2074. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/testing/_internal/distributed/rpc/dist_autograd_test.py:483
- **Description:** exec()
- **Evidence (Line):** `self._verify_graph_for_rpc_call_exec(list(send_functions.values())[1])`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2075. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/testing/_internal/distributed/rpc/dist_autograd_test.py:947
- **Description:** exec()
- **Evidence (Line):** `def _verify_graph_for_rpc_call_exec(self, send_function):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2076. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/testing/_internal/jit_metaprogramming_utils.py:627
- **Description:** eval()
- **Evidence (Line):** `# eval() is not supported, so skip these tests`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2077. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/testing/_internal/jit_utils.py:56
- **Description:** exec()
- **Evidence (Line):** `exec(code, glob, loc)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2078. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/testing/_internal/jit_utils.py:216
- **Description:** pickle load/dumps
- **Evidence (Line):** `debug_files.append(pickle.load(stream))`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 2079. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/utils/_config_module.py:867
- **Description:** pickle load/dumps
- **Evidence (Line):** `config = pickle.loads(maybe_pickled_config)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 2080. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/utils/_sympy/functions.py:234
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, base: sympy.Integer, divisor: sympy.Integer) -> sympy.Basic | None:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2081. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/utils/_sympy/functions.py:339
- **Description:** eval()
- **Evidence (Line):** `def eval(`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2082. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/utils/_sympy/functions.py:417
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, c: sympy.Basic, p: sympy.Basic, q: sympy.Basic) -> sympy.Basic | None:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2083. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/utils/_sympy/functions.py:433
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, p: sympy.Expr, q: sympy.Expr) -> sympy.Expr | None:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2084. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/utils/_sympy/functions.py:501
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, p: sympy.Expr, q: sympy.Expr) -> sympy.Expr | None:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2085. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/utils/_sympy/functions.py:560
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, number: sympy.Expr) -> sympy.Basic | None:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2086. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/utils/_sympy/functions.py:579
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, number: sympy.Expr) -> sympy.Basic | None:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2087. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/utils/_sympy/functions.py:615
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, base: sympy.Expr, shift: sympy.Expr) -> sympy.Basic:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2088. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/utils/_sympy/functions.py:625
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, base: sympy.Expr, shift: sympy.Expr) -> sympy.Basic:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2089. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/utils/_sympy/functions.py:1123
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, base: sympy.Expr, exp: sympy.Expr) -> sympy.Basic | None:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2090. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/utils/_sympy/functions.py:1151
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, base: sympy.Expr, exp: sympy.Expr) -> sympy.Basic | None:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2091. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/utils/_sympy/functions.py:1174
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, base: sympy.Expr, divisor: sympy.Expr) -> sympy.Basic | None:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2092. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/utils/_sympy/functions.py:1200
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, base: sympy.Expr, divisor: sympy.Expr) -> sympy.Basic | None:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2093. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/utils/_sympy/functions.py:1232
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, *args: sympy.Expr) -> int | None:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2094. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/utils/_sympy/functions.py:1293
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, number: sympy.Expr) -> sympy.Basic | None:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2095. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/utils/_sympy/functions.py:1309
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, number: sympy.Expr) -> sympy.Basic | None:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2096. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/utils/_sympy/functions.py:1329
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, number: sympy.Expr) -> sympy.Basic | None:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2097. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/utils/_sympy/functions.py:1360
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, number: sympy.Expr, ndigits: sympy.Expr) -> sympy.Basic | None:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2098. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/utils/_sympy/functions.py:1372
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, number: sympy.Expr) -> sympy.Basic | None:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2099. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/utils/_sympy/functions.py:1481
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, a: sympy.Expr) -> sympy.Basic | None:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2100. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/utils/_sympy/functions.py:1546
- **Description:** eval()
- **Evidence (Line):** `def eval(cls, a: sympy.Expr, b: sympy.Expr) -> sympy.Basic | None:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2101. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/utils/_traceback.py:112
- **Description:** eval()
- **Evidence (Line):** `fake_frame = eval(`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2102. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/utils/benchmark/examples/compare.py:86
- **Description:** pickle load/dumps
- **Evidence (Line):** `pickle.loads(i) for i in serialized_results`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 2103. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/utils/benchmark/utils/valgrind_wrapper/timer_interface.py:379
- **Description:** pickle load/dumps
- **Evidence (Line):** `globals={"counter": pickle.loads(pickle.dumps(counter))}`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 2104. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/utils/benchmark/utils/valgrind_wrapper/timer_interface.py:461
- **Description:** pickle load/dumps
- **Evidence (Line):** `f"with open({repr(path)}, 'rb') as f:\n    {name} = pickle.load(f)")`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 2105. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/utils/data/datapipes/datapipe.py:396
- **Description:** pickle load/dumps
- **Evidence (Line):** `self._datapipe = pickle.loads(value)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 2106. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/utils/data/datapipes/utils/decoder.py:74
- **Description:** pickle load/dumps
- **Evidence (Line):** `return pickle.loads(data)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 2107. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/utils/mobile_optimizer.py:106
- **Description:** eval()
- **Evidence (Line):** `"message": f"Operator {op_name} exists, remember to call eval() before "`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2108. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/utils/mobile_optimizer.py:111
- **Description:** eval()
- **Evidence (Line):** `"message": f"Operator {op_name} exists, remember to call eval() before "`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2109. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/utils/model_dump/__init__.py:292
- **Description:** pickle load/dumps
- **Evidence (Line):** `debug_info_t = pickle.loads(raw_debug)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 2110. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/utils/tensorboard/_pytorch_graph.py:326
- **Description:** eval()
- **Evidence (Line):** `with _set_model_to_eval(model):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2111. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/utils/tensorboard/_pytorch_graph.py:358
- **Description:** eval()
- **Evidence (Line):** `def _set_model_to_eval(model):`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2112. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/xpu/graphs.py:142
- **Description:** exec()
- **Evidence (Line):** `def raw_xpu_graph_exec(self) -> int:`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2113. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/xpu/graphs.py:143
- **Description:** exec()
- **Evidence (Line):** `r"""Returns the underlying xpuGraphExec_t. ``instantiate`` must have been called if ``keep_graph`` i`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2114. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/xpu/graphs.py:147
- **Description:** exec()
- **Evidence (Line):** `return super().raw_xpu_graph_exec()`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2115. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/torchgen/gen.py:247
- **Description:** yaml.load (unsafe)
- **Evidence (Line):** `es = yaml.load(f, Loader=LineLoader)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 2116. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/torchgen/gen.py:268
- **Description:** yaml.load (unsafe)
- **Evidence (Line):** `es = yaml.load(f, Loader=LineLoader)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 2117. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/torchgen/gen_backend_stubs.py:57
- **Description:** yaml.load (unsafe)
- **Evidence (Line):** `yaml_values = yaml.load(f, Loader=YamlLoader)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 2118. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/torchgen/gen_lazy_tensor.py:103
- **Description:** yaml.load (unsafe)
- **Evidence (Line):** `yaml_values = yaml.load(f, Loader=YamlLoader)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 2119. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/torchgen/packaged/autograd/gen_python_functions.py:609
- **Description:** yaml.load (unsafe)
- **Evidence (Line):** `deprecated_defs = yaml.load(f, Loader=YamlLoader)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 2120. [Critical] Insecure Deserialization
- **File:** .pytest_venv/lib/python3.11/site-packages/torchgen/packaged/autograd/load_derivatives.py:115
- **Description:** yaml.load (unsafe)
- **Evidence (Line):** `definitions = yaml.load(f, Loader=YamlLoader)`
- **Exploit Scenario:** Attacker could achieve remote code execution
- **Fix Recommendation:** Use safe serialization formats (JSON), avoid pickle/yaml.load with untrusted data
- **Confidence:** High

### 2121. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/tqdm/cli.py:38
- **Description:** eval()
- **Evidence (Line):** `return literal_eval(f'"{val}"').encode()`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2122. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/tqdm/cli.py:301
- **Description:** eval()
- **Evidence (Line):** `t.update(literal_eval(i.decode()))`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2123. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/tqdm/cli.py:304
- **Description:** eval()
- **Evidence (Line):** `t.update(literal_eval(i.decode()) - t.n)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2124. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/tqdm/cli.py:317
- **Description:** eval()
- **Evidence (Line):** `t.update(literal_eval(i.decode()))`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2125. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/tqdm/cli.py:320
- **Description:** eval()
- **Evidence (Line):** `t.update(literal_eval(i.decode()) - t.n)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2126. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/typing_extensions.py:1591
- **Description:** eval()
- **Evidence (Line):** `(unless you are familiar with how eval() and exec() work).  The`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2127. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/typing_extensions.py:4167
- **Description:** eval()
- **Evidence (Line):** `# as a way of emulating annotation scopes when calling `eval()``
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2128. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/typing_extensions.py:4172
- **Description:** eval()
- **Evidence (Line):** `value if not isinstance(value, str) else eval(value, globals, locals)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2129. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/typing_extensions.py:4219
- **Description:** eval()
- **Evidence (Line):** `# If we pass None to eval() below, the globals of this module are used.`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2130. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/typing_extensions.py:4231
- **Description:** eval()
- **Evidence (Line):** `# as a way of emulating annotation scopes when calling `eval()``
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2131. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/typing_extensions.py:4254
- **Description:** eval()
- **Evidence (Line):** `value = eval(code, globals, locals)`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2132. [Critical] Command Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/typing_extensions.py:1591
- **Description:** exec()
- **Evidence (Line):** `(unless you are familiar with how eval() and exec() work).  The`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2133. [Critical] Command Injection
- **File:** .pytest_venv/share/jupyter/labextensions/jupyterlab-plotly/static/0.03957c7088ad5ed0.js:3526
- **Description:** eval()
- **Evidence (Line):** ``),w=c.createShader(c.FRAGMENT_SHADER);if(c.isContextLost()){this.failedToCreate=!0;return}c.shaderS`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2134. [Critical] Command Injection
- **File:** .pytest_venv/share/jupyter/labextensions/jupyterlab-plotly/static/0.03957c7088ad5ed0.js:1
- **Description:** exec()
- **Evidence (Line):** `(self.rspackChunkjupyterlab_plotly=self.rspackChunkjupyterlab_plotly||[]).push([[0],{2(t,r,i){"use s`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2135. [Critical] Command Injection
- **File:** .pytest_venv/share/jupyter/labextensions/jupyterlab-plotly/static/0.03957c7088ad5ed0.js:4
- **Description:** exec()
- **Evidence (Line):** ``)},a},t.csv=t.dsv(",","text/csv"),t.tsv=t.dsv("	","text/tab-separated-values");var te,tt,tr,ti,tn=t`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2136. [Critical] Command Injection
- **File:** .pytest_venv/share/jupyter/labextensions/jupyterlab-plotly/static/0.03957c7088ad5ed0.js:4
- **Description:** exec()
- **Evidence (Line):** ``)},a},t.csv=t.dsv(",","text/csv"),t.tsv=t.dsv("	","text/tab-separated-values");var te,tt,tr,ti,tn=t`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2137. [Critical] Command Injection
- **File:** .pytest_venv/share/jupyter/labextensions/jupyterlab-plotly/static/0.03957c7088ad5ed0.js:4
- **Description:** exec()
- **Evidence (Line):** ``)},a},t.csv=t.dsv(",","text/csv"),t.tsv=t.dsv("	","text/tab-separated-values");var te,tt,tr,ti,tn=t`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2138. [Critical] Command Injection
- **File:** .pytest_venv/share/jupyter/labextensions/jupyterlab-plotly/static/0.03957c7088ad5ed0.js:4
- **Description:** exec()
- **Evidence (Line):** ``)},a},t.csv=t.dsv(",","text/csv"),t.tsv=t.dsv("	","text/tab-separated-values");var te,tt,tr,ti,tn=t`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2139. [Critical] Command Injection
- **File:** .pytest_venv/share/jupyter/labextensions/jupyterlab-plotly/static/0.03957c7088ad5ed0.js:4
- **Description:** exec()
- **Evidence (Line):** ``)},a},t.csv=t.dsv(",","text/csv"),t.tsv=t.dsv("	","text/tab-separated-values");var te,tt,tr,ti,tn=t`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2140. [Critical] Command Injection
- **File:** .pytest_venv/share/jupyter/labextensions/jupyterlab-plotly/static/0.03957c7088ad5ed0.js:4
- **Description:** exec()
- **Evidence (Line):** ``)},a},t.csv=t.dsv(",","text/csv"),t.tsv=t.dsv("	","text/tab-separated-values");var te,tt,tr,ti,tn=t`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2141. [Critical] Command Injection
- **File:** .pytest_venv/share/jupyter/labextensions/jupyterlab-plotly/static/0.03957c7088ad5ed0.js:4
- **Description:** exec()
- **Evidence (Line):** ``)},a},t.csv=t.dsv(",","text/csv"),t.tsv=t.dsv("	","text/tab-separated-values");var te,tt,tr,ti,tn=t`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2142. [Critical] Command Injection
- **File:** .pytest_venv/share/jupyter/labextensions/jupyterlab-plotly/static/0.03957c7088ad5ed0.js:4
- **Description:** exec()
- **Evidence (Line):** ``)},a},t.csv=t.dsv(",","text/csv"),t.tsv=t.dsv("	","text/tab-separated-values");var te,tt,tr,ti,tn=t`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2143. [Critical] Command Injection
- **File:** .pytest_venv/share/jupyter/labextensions/jupyterlab-plotly/static/0.03957c7088ad5ed0.js:4
- **Description:** exec()
- **Evidence (Line):** ``)},a},t.csv=t.dsv(",","text/csv"),t.tsv=t.dsv("	","text/tab-separated-values");var te,tt,tr,ti,tn=t`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2144. [Critical] Command Injection
- **File:** .pytest_venv/share/jupyter/labextensions/jupyterlab-plotly/static/0.03957c7088ad5ed0.js:4
- **Description:** exec()
- **Evidence (Line):** ``)},a},t.csv=t.dsv(",","text/csv"),t.tsv=t.dsv("	","text/tab-separated-values");var te,tt,tr,ti,tn=t`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2145. [Critical] Command Injection
- **File:** .pytest_venv/share/jupyter/labextensions/jupyterlab-plotly/static/0.03957c7088ad5ed0.js:4
- **Description:** exec()
- **Evidence (Line):** ``)},a},t.csv=t.dsv(",","text/csv"),t.tsv=t.dsv("	","text/tab-separated-values");var te,tt,tr,ti,tn=t`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2146. [Critical] Command Injection
- **File:** .pytest_venv/share/jupyter/labextensions/jupyterlab-plotly/static/0.03957c7088ad5ed0.js:4
- **Description:** exec()
- **Evidence (Line):** ``)},a},t.csv=t.dsv(",","text/csv"),t.tsv=t.dsv("	","text/tab-separated-values");var te,tt,tr,ti,tn=t`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2147. [Critical] Command Injection
- **File:** .pytest_venv/share/jupyter/labextensions/jupyterlab-plotly/static/0.03957c7088ad5ed0.js:4
- **Description:** exec()
- **Evidence (Line):** ``)},a},t.csv=t.dsv(",","text/csv"),t.tsv=t.dsv("	","text/tab-separated-values");var te,tt,tr,ti,tn=t`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2148. [Critical] Command Injection
- **File:** .pytest_venv/share/jupyter/labextensions/jupyterlab-plotly/static/0.03957c7088ad5ed0.js:4
- **Description:** exec()
- **Evidence (Line):** ``)},a},t.csv=t.dsv(",","text/csv"),t.tsv=t.dsv("	","text/tab-separated-values");var te,tt,tr,ti,tn=t`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2149. [Critical] Command Injection
- **File:** .pytest_venv/share/jupyter/labextensions/jupyterlab-plotly/static/0.03957c7088ad5ed0.js:4
- **Description:** exec()
- **Evidence (Line):** ``)},a},t.csv=t.dsv(",","text/csv"),t.tsv=t.dsv("	","text/tab-separated-values");var te,tt,tr,ti,tn=t`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2150. [Critical] Command Injection
- **File:** .pytest_venv/share/jupyter/labextensions/jupyterlab-plotly/static/0.03957c7088ad5ed0.js:4
- **Description:** exec()
- **Evidence (Line):** ``)},a},t.csv=t.dsv(",","text/csv"),t.tsv=t.dsv("	","text/tab-separated-values");var te,tt,tr,ti,tn=t`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2151. [Critical] Command Injection
- **File:** .pytest_venv/share/jupyter/labextensions/jupyterlab-plotly/static/0.03957c7088ad5ed0.js:4
- **Description:** exec()
- **Evidence (Line):** ``)},a},t.csv=t.dsv(",","text/csv"),t.tsv=t.dsv("	","text/tab-separated-values");var te,tt,tr,ti,tn=t`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2152. [Critical] Command Injection
- **File:** .pytest_venv/share/jupyter/labextensions/jupyterlab-plotly/static/0.03957c7088ad5ed0.js:4
- **Description:** exec()
- **Evidence (Line):** ``)},a},t.csv=t.dsv(",","text/csv"),t.tsv=t.dsv("	","text/tab-separated-values");var te,tt,tr,ti,tn=t`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2153. [Critical] Command Injection
- **File:** .pytest_venv/share/jupyter/labextensions/jupyterlab-plotly/static/0.03957c7088ad5ed0.js:4
- **Description:** exec()
- **Evidence (Line):** ``)},a},t.csv=t.dsv(",","text/csv"),t.tsv=t.dsv("	","text/tab-separated-values");var te,tt,tr,ti,tn=t`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2154. [Critical] Command Injection
- **File:** .pytest_venv/share/jupyter/labextensions/jupyterlab-plotly/static/0.03957c7088ad5ed0.js:4
- **Description:** exec()
- **Evidence (Line):** ``)},a},t.csv=t.dsv(",","text/csv"),t.tsv=t.dsv("	","text/tab-separated-values");var te,tt,tr,ti,tn=t`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2155. [Critical] Command Injection
- **File:** .pytest_venv/share/jupyter/labextensions/jupyterlab-plotly/static/0.03957c7088ad5ed0.js:4
- **Description:** exec()
- **Evidence (Line):** ``)},a},t.csv=t.dsv(",","text/csv"),t.tsv=t.dsv("	","text/tab-separated-values");var te,tt,tr,ti,tn=t`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2156. [Critical] Command Injection
- **File:** .pytest_venv/share/jupyter/labextensions/jupyterlab-plotly/static/0.03957c7088ad5ed0.js:4
- **Description:** exec()
- **Evidence (Line):** ``)},a},t.csv=t.dsv(",","text/csv"),t.tsv=t.dsv("	","text/tab-separated-values");var te,tt,tr,ti,tn=t`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2157. [Critical] Command Injection
- **File:** .pytest_venv/share/jupyter/labextensions/jupyterlab-plotly/static/0.03957c7088ad5ed0.js:4
- **Description:** exec()
- **Evidence (Line):** ``)},a},t.csv=t.dsv(",","text/csv"),t.tsv=t.dsv("	","text/tab-separated-values");var te,tt,tr,ti,tn=t`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2158. [Critical] Command Injection
- **File:** .pytest_venv/share/jupyter/labextensions/jupyterlab-plotly/static/0.03957c7088ad5ed0.js:4
- **Description:** exec()
- **Evidence (Line):** ``)},a},t.csv=t.dsv(",","text/csv"),t.tsv=t.dsv("	","text/tab-separated-values");var te,tt,tr,ti,tn=t`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2159. [Critical] Command Injection
- **File:** .pytest_venv/share/jupyter/labextensions/jupyterlab-plotly/static/0.03957c7088ad5ed0.js:4
- **Description:** exec()
- **Evidence (Line):** ``)},a},t.csv=t.dsv(",","text/csv"),t.tsv=t.dsv("	","text/tab-separated-values");var te,tt,tr,ti,tn=t`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2160. [Critical] Command Injection
- **File:** .pytest_venv/share/jupyter/labextensions/jupyterlab-plotly/static/0.03957c7088ad5ed0.js:4
- **Description:** exec()
- **Evidence (Line):** ``)},a},t.csv=t.dsv(",","text/csv"),t.tsv=t.dsv("	","text/tab-separated-values");var te,tt,tr,ti,tn=t`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2161. [Critical] Command Injection
- **File:** .pytest_venv/share/jupyter/labextensions/jupyterlab-plotly/static/0.03957c7088ad5ed0.js:4
- **Description:** exec()
- **Evidence (Line):** ``)},a},t.csv=t.dsv(",","text/csv"),t.tsv=t.dsv("	","text/tab-separated-values");var te,tt,tr,ti,tn=t`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2162. [Critical] Command Injection
- **File:** .pytest_venv/share/jupyter/labextensions/jupyterlab-plotly/static/0.03957c7088ad5ed0.js:4
- **Description:** exec()
- **Evidence (Line):** ``)},a},t.csv=t.dsv(",","text/csv"),t.tsv=t.dsv("	","text/tab-separated-values");var te,tt,tr,ti,tn=t`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2163. [Critical] Command Injection
- **File:** .pytest_venv/share/jupyter/labextensions/jupyterlab-plotly/static/0.03957c7088ad5ed0.js:9
- **Description:** exec()
- **Evidence (Line):** ``+l.year}return I(r,t,n,o)};var F=3*c;t.incrementMonth=function(t,r,i){i=_(i)&&i;var n=s(t,c);if(t=M`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2164. [Critical] Command Injection
- **File:** .pytest_venv/share/jupyter/labextensions/jupyterlab-plotly/static/0.03957c7088ad5ed0.js:85
- **Description:** exec()
- **Evidence (Line):** ``))}throw a}},T.match=function t(r,i,n){O(r,i,n,t,"match")},T.doesNotMatch=function t(r,i,n){O(r,i,n`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2165. [Critical] Command Injection
- **File:** .pytest_venv/share/jupyter/labextensions/jupyterlab-plotly/static/0.03957c7088ad5ed0.js:89
- **Description:** exec()
- **Evidence (Line):** `children sum =`,a].join(" "))}t.value=i})}else!function t(r,i,n){var a=0,o=r.children;if(o){for(var `
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2166. [Critical] Command Injection
- **File:** .pytest_venv/share/jupyter/labextensions/jupyterlab-plotly/static/0.03957c7088ad5ed0.js:89
- **Description:** exec()
- **Evidence (Line):** `children sum =`,a].join(" "))}t.value=i})}else!function t(r,i,n){var a=0,o=r.children;if(o){for(var `
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2167. [Critical] Command Injection
- **File:** .pytest_venv/share/jupyter/labextensions/jupyterlab-plotly/static/0.03957c7088ad5ed0.js:89
- **Description:** exec()
- **Evidence (Line):** `children sum =`,a].join(" "))}t.value=i})}else!function t(r,i,n){var a=0,o=r.children;if(o){for(var `
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2168. [Critical] Command Injection
- **File:** .pytest_venv/share/jupyter/labextensions/jupyterlab-plotly/static/0.03957c7088ad5ed0.js:89
- **Description:** exec()
- **Evidence (Line):** `children sum =`,a].join(" "))}t.value=i})}else!function t(r,i,n){var a=0,o=r.children;if(o){for(var `
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2169. [Critical] Command Injection
- **File:** .pytest_venv/share/jupyter/labextensions/jupyterlab-plotly/static/0.03957c7088ad5ed0.js:89
- **Description:** exec()
- **Evidence (Line):** `children sum =`,a].join(" "))}t.value=i})}else!function t(r,i,n){var a=0,o=r.children;if(o){for(var `
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2170. [Critical] Command Injection
- **File:** .pytest_venv/share/jupyter/labextensions/jupyterlab-plotly/static/0.03957c7088ad5ed0.js:89
- **Description:** exec()
- **Evidence (Line):** `children sum =`,a].join(" "))}t.value=i})}else!function t(r,i,n){var a=0,o=r.children;if(o){for(var `
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2171. [Critical] Command Injection
- **File:** .pytest_venv/share/jupyter/labextensions/jupyterlab-plotly/static/0.03957c7088ad5ed0.js:89
- **Description:** exec()
- **Evidence (Line):** `children sum =`,a].join(" "))}t.value=i})}else!function t(r,i,n){var a=0,o=r.children;if(o){for(var `
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2172. [Critical] Command Injection
- **File:** .pytest_venv/share/jupyter/labextensions/jupyterlab-plotly/static/0.03957c7088ad5ed0.js:89
- **Description:** exec()
- **Evidence (Line):** `children sum =`,a].join(" "))}t.value=i})}else!function t(r,i,n){var a=0,o=r.children;if(o){for(var `
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2173. [Critical] Command Injection
- **File:** .pytest_venv/share/jupyter/labextensions/jupyterlab-plotly/static/0.03957c7088ad5ed0.js:89
- **Description:** exec()
- **Evidence (Line):** `children sum =`,a].join(" "))}t.value=i})}else!function t(r,i,n){var a=0,o=r.children;if(o){for(var `
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2174. [Critical] Command Injection
- **File:** .pytest_venv/share/jupyter/labextensions/jupyterlab-plotly/static/0.03957c7088ad5ed0.js:1994
- **Description:** exec()
- **Evidence (Line):** ``)&&"\\"!==i?(k(p.join("")),f=999,h):(p.push(r),i=r,h+1)}function E(t){for(var r,i,n=0;;){if(r=a.ind`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2175. [Critical] Command Injection
- **File:** .pytest_venv/share/jupyter/labextensions/jupyterlab-plotly/static/0.03957c7088ad5ed0.js:1994
- **Description:** exec()
- **Evidence (Line):** ``)&&"\\"!==i?(k(p.join("")),f=999,h):(p.push(r),i=r,h+1)}function E(t){for(var r,i,n=0;;){if(r=a.ind`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2176. [Critical] Command Injection
- **File:** .pytest_venv/share/jupyter/labextensions/jupyterlab-plotly/static/0.03957c7088ad5ed0.js:1994
- **Description:** exec()
- **Evidence (Line):** ``)&&"\\"!==i?(k(p.join("")),f=999,h):(p.push(r),i=r,h+1)}function E(t){for(var r,i,n=0;;){if(r=a.ind`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2177. [Critical] Command Injection
- **File:** .pytest_venv/share/jupyter/labextensions/jupyterlab-plotly/static/0.03957c7088ad5ed0.js:1994
- **Description:** exec()
- **Evidence (Line):** ``)&&"\\"!==i?(k(p.join("")),f=999,h):(p.push(r),i=r,h+1)}function E(t){for(var r,i,n=0;;){if(r=a.ind`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2178. [Critical] Command Injection
- **File:** .pytest_venv/share/jupyter/labextensions/jupyterlab-plotly/static/0.03957c7088ad5ed0.js:1994
- **Description:** exec()
- **Evidence (Line):** ``)&&"\\"!==i?(k(p.join("")),f=999,h):(p.push(r),i=r,h+1)}function E(t){for(var r,i,n=0;;){if(r=a.ind`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2179. [Critical] Command Injection
- **File:** .pytest_venv/share/jupyter/labextensions/jupyterlab-plotly/static/0.03957c7088ad5ed0.js:1994
- **Description:** exec()
- **Evidence (Line):** ``)&&"\\"!==i?(k(p.join("")),f=999,h):(p.push(r),i=r,h+1)}function E(t){for(var r,i,n=0;;){if(r=a.ind`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2180. [Critical] Command Injection
- **File:** .pytest_venv/share/jupyter/labextensions/jupyterlab-plotly/static/0.03957c7088ad5ed0.js:1997
- **Description:** exec()
- **Evidence (Line):** ``;for(_=0,w=x*m,T=n,M="",b=0;b<E.length;++b){var D=b+A<h.length?h[b+A]:h[h.length-1];l===D?M+=E[b]:(`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2181. [Critical] Command Injection
- **File:** .pytest_venv/share/jupyter/labextensions/jupyterlab-plotly/static/0.03957c7088ad5ed0.js:2618
- **Description:** exec()
- **Evidence (Line):** ``,uniforms:{range:t.prop("range"),lineWidth:t.prop("lineWidth"),capSize:t.prop("capSize"),opacity:t.`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2182. [Critical] Command Injection
- **File:** .pytest_venv/share/jupyter/labextensions/jupyterlab-plotly/static/0.03957c7088ad5ed0.js:2618
- **Description:** exec()
- **Evidence (Line):** ``,uniforms:{range:t.prop("range"),lineWidth:t.prop("lineWidth"),capSize:t.prop("capSize"),opacity:t.`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2183. [Critical] Command Injection
- **File:** .pytest_venv/share/jupyter/labextensions/jupyterlab-plotly/static/0.03957c7088ad5ed0.js:2710
- **Description:** exec()
- **Evidence (Line):** ``)),x.push({type:"Feature",geometry:{type:"Point",coordinates:_.lonlat},properties:{symbol:h(b),angl`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2184. [Critical] Command Injection
- **File:** .pytest_venv/share/jupyter/labextensions/jupyterlab-plotly/static/0.03957c7088ad5ed0.js:2762
- **Description:** exec()
- **Evidence (Line):** `${l.shaderPreludeCode.vertexSource}`,define:l.shaderDefine},defaultProjectionData:c,getProjectionDat`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2185. [Critical] Command Injection
- **File:** .pytest_venv/share/jupyter/labextensions/jupyterlab-plotly/static/0.03957c7088ad5ed0.js:2762
- **Description:** exec()
- **Evidence (Line):** `${l.shaderPreludeCode.vertexSource}`,define:l.shaderDefine},defaultProjectionData:c,getProjectionDat`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2186. [Critical] Command Injection
- **File:** .pytest_venv/share/jupyter/labextensions/jupyterlab-plotly/static/0.03957c7088ad5ed0.js:3526
- **Description:** exec()
- **Evidence (Line):** ``),w=c.createShader(c.FRAGMENT_SHADER);if(c.isContextLost()){this.failedToCreate=!0;return}c.shaderS`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2187. [Critical] Command Injection
- **File:** .pytest_venv/share/jupyter/labextensions/jupyterlab-plotly/static/0.03957c7088ad5ed0.js:3526
- **Description:** exec()
- **Evidence (Line):** ``),w=c.createShader(c.FRAGMENT_SHADER);if(c.isContextLost()){this.failedToCreate=!0;return}c.shaderS`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2188. [Critical] Command Injection
- **File:** .pytest_venv/share/jupyter/labextensions/jupyterlab-plotly/static/0.03957c7088ad5ed0.js:3526
- **Description:** exec()
- **Evidence (Line):** ``),w=c.createShader(c.FRAGMENT_SHADER);if(c.isContextLost()){this.failedToCreate=!0;return}c.shaderS`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2189. [Critical] Command Injection
- **File:** .pytest_venv/share/jupyter/labextensions/jupyterlab-plotly/static/0.03957c7088ad5ed0.js:3526
- **Description:** exec()
- **Evidence (Line):** ``),w=c.createShader(c.FRAGMENT_SHADER);if(c.isContextLost()){this.failedToCreate=!0;return}c.shaderS`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2190. [Critical] Command Injection
- **File:** .pytest_venv/share/jupyter/labextensions/jupyterlab-plotly/static/remoteEntry.1dda3c8f6e5cd2a3.js:1
- **Description:** exec()
- **Evidence (Line):** `var _JUPYTERLAB;(()=>{"use strict";var e,n,r,t,o,i,u,a,c,l,f,s,p,h,d,v,g,y,m,b,S,j,w,E,k,x,T,P,A,C,O`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2191. [Critical] Command Injection
- **File:** .pytest_venv/share/jupyter/labextensions/jupyterlab-plotly/static/remoteEntry.1dda3c8f6e5cd2a3.js:1
- **Description:** exec()
- **Evidence (Line):** `var _JUPYTERLAB;(()=>{"use strict";var e,n,r,t,o,i,u,a,c,l,f,s,p,h,d,v,g,y,m,b,S,j,w,E,k,x,T,P,A,C,O`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2192. [Critical] Command Injection
- **File:** .pytest_venv/share/jupyter/labextensions/jupyterlab-plotly/static/remoteEntry.1dda3c8f6e5cd2a3.js:1
- **Description:** exec()
- **Evidence (Line):** `var _JUPYTERLAB;(()=>{"use strict";var e,n,r,t,o,i,u,a,c,l,f,s,p,h,d,v,g,y,m,b,S,j,w,E,k,x,T,P,A,C,O`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2193. [Critical] Command Injection
- **File:** .pytest_venv/share/jupyter/labextensions/jupyterlab-plotly/static/remoteEntry.1dda3c8f6e5cd2a3.js:1
- **Description:** exec()
- **Evidence (Line):** `var _JUPYTERLAB;(()=>{"use strict";var e,n,r,t,o,i,u,a,c,l,f,s,p,h,d,v,g,y,m,b,S,j,w,E,k,x,T,P,A,C,O`
- **Exploit Scenario:** Attacker could execute arbitrary system commands
- **Fix Recommendation:** Avoid shell execution, use subprocess with shell=False and list args
- **Confidence:** High

### 2194. [High] SQL Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/io/sql.py:2381
- **Description:** SQL f-string interpolation
- **Evidence (Line):** `cur.execute(f"DROP TABLE {table_name}")`
- **Exploit Scenario:** Attacker could manipulate SQL queries to access/modify/delete data
- **Fix Recommendation:** Use parameterized queries/prepared statements
- **Confidence:** Medium

### 2195. [High] SQL Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/io/test_sql.py:566
- **Description:** SQL f-string interpolation
- **Evidence (Line):** `conn.execute(f"DROP TABLE IF EXISTS {sql._get_valid_sqlite_name(table_name)}")`
- **Exploit Scenario:** Attacker could manipulate SQL queries to access/modify/delete data
- **Fix Recommendation:** Use parameterized queries/prepared statements
- **Confidence:** Medium

### 2196. [High] SQL Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/io/test_sql.py:573
- **Description:** SQL f-string interpolation
- **Evidence (Line):** `cur.execute(f'DROP TABLE IF EXISTS "{table_name}"')`
- **Exploit Scenario:** Attacker could manipulate SQL queries to access/modify/delete data
- **Fix Recommendation:** Use parameterized queries/prepared statements
- **Confidence:** Medium

### 2197. [High] SQL Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/io/test_sql.py:587
- **Description:** SQL f-string interpolation
- **Evidence (Line):** `conn.execute(f"DROP VIEW IF EXISTS {sql._get_valid_sqlite_name(view_name)}")`
- **Exploit Scenario:** Attacker could manipulate SQL queries to access/modify/delete data
- **Fix Recommendation:** Use parameterized queries/prepared statements
- **Confidence:** Medium

### 2198. [High] SQL Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/io/test_sql.py:593
- **Description:** SQL f-string interpolation
- **Evidence (Line):** `cur.execute(f'DROP VIEW IF EXISTS "{view_name}"')`
- **Exploit Scenario:** Attacker could manipulate SQL queries to access/modify/delete data
- **Fix Recommendation:** Use parameterized queries/prepared statements
- **Confidence:** Medium

### 2199. [High] SQL Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/io/test_sql.py:4043
- **Description:** SQL f-string interpolation
- **Evidence (Line):** `recs = conn.execute(f"PRAGMA table_info({table})")`
- **Exploit Scenario:** Attacker could manipulate SQL queries to access/modify/delete data
- **Fix Recommendation:** Use parameterized queries/prepared statements
- **Confidence:** Medium

### 2200. [High] SQL Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/plotly/labextension/static/0.03957c7088ad5ed0.js:3526
- **Description:** SQL string concatenation
- **Evidence (Line):** ``),w=c.createShader(c.FRAGMENT_SHADER);if(c.isContextLost()){this.failedToCreate=!0;return}c.shaderS`
- **Exploit Scenario:** Attacker could manipulate SQL queries to access/modify/delete data
- **Fix Recommendation:** Use parameterized queries/prepared statements
- **Confidence:** Medium

### 2201. [High] SQL Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/dialects/mssql/base.py:3199
- **Description:** SQL f-string interpolation
- **Evidence (Line):** `cursor.execute(f"SET TRANSACTION ISOLATION LEVEL {level}")`
- **Exploit Scenario:** Attacker could manipulate SQL queries to access/modify/delete data
- **Fix Recommendation:** Use parameterized queries/prepared statements
- **Confidence:** Medium

### 2202. [High] SQL Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/dialects/mssql/base.py:1992
- **Description:** SQL % formatting
- **Evidence (Line):** `self.cursor.execute(`
- **Exploit Scenario:** Attacker could manipulate SQL queries to access/modify/delete data
- **Fix Recommendation:** Use parameterized queries/prepared statements
- **Confidence:** Medium

### 2203. [High] SQL Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/dialects/mysql/base.py:2816
- **Description:** SQL f-string interpolation
- **Evidence (Line):** `cursor.execute(f"SET SESSION TRANSACTION ISOLATION LEVEL {level}")`
- **Exploit Scenario:** Attacker could manipulate SQL queries to access/modify/delete data
- **Fix Recommendation:** Use parameterized queries/prepared statements
- **Confidence:** Medium

### 2204. [High] SQL Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/dialects/mysql/mysqldb.py:181
- **Description:** SQL % formatting
- **Evidence (Line):** `cursor.execute("SET NAMES %s" % charset_name)`
- **Exploit Scenario:** Attacker could manipulate SQL queries to access/modify/delete data
- **Fix Recommendation:** Use parameterized queries/prepared statements
- **Confidence:** Medium

### 2205. [High] SQL Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/dialects/oracle/cx_oracle.py:1236
- **Description:** SQL f-string interpolation
- **Evidence (Line):** `cursor.execute(f"ALTER SESSION SET ISOLATION_LEVEL={level}")`
- **Exploit Scenario:** Attacker could manipulate SQL queries to access/modify/delete data
- **Fix Recommendation:** Use parameterized queries/prepared statements
- **Confidence:** Medium

### 2206. [High] SQL Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/dialects/oracle/provision.py:286
- **Description:** SQL % formatting
- **Evidence (Line):** `cursor.execute("ALTER SESSION SET CURRENT_SCHEMA=%s" % schema_name)`
- **Exploit Scenario:** Attacker could manipulate SQL queries to access/modify/delete data
- **Fix Recommendation:** Use parameterized queries/prepared statements
- **Confidence:** Medium

### 2207. [High] SQL Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/dialects/postgresql/base.py:322
- **Description:** SQL % formatting
- **Evidence (Line):** `cursor.execute("SET SESSION search_path='%s'" % schema_name)`
- **Exploit Scenario:** Attacker could manipulate SQL queries to access/modify/delete data
- **Fix Recommendation:** Use parameterized queries/prepared statements
- **Confidence:** Medium

### 2208. [High] SQL Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/dialects/postgresql/pg8000.py:372
- **Description:** SQL string concatenation
- **Evidence (Line):** `self.cursor.execute(`
- **Exploit Scenario:** Attacker could manipulate SQL queries to access/modify/delete data
- **Fix Recommendation:** Use parameterized queries/prepared statements
- **Confidence:** Medium

### 2209. [High] SQL Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/dialects/postgresql/pg8000.py:591
- **Description:** SQL f-string interpolation
- **Evidence (Line):** `cursor.execute(f"""`
- **Exploit Scenario:** Attacker could manipulate SQL queries to access/modify/delete data
- **Fix Recommendation:** Use parameterized queries/prepared statements
- **Confidence:** Medium

### 2210. [High] SQL Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/dialects/postgresql/pg8000.py:550
- **Description:** SQL % formatting
- **Evidence (Line):** `cursor.execute(`
- **Exploit Scenario:** Attacker could manipulate SQL queries to access/modify/delete data
- **Fix Recommendation:** Use parameterized queries/prepared statements
- **Confidence:** Medium

### 2211. [High] SQL Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/dialects/postgresql/pg8000.py:571
- **Description:** SQL % formatting
- **Evidence (Line):** `cursor.execute(`
- **Exploit Scenario:** Attacker could manipulate SQL queries to access/modify/delete data
- **Fix Recommendation:** Use parameterized queries/prepared statements
- **Confidence:** Medium

### 2212. [High] SQL Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/dialects/postgresql/provision.py:89
- **Description:** SQL % formatting
- **Evidence (Line):** `cursor.execute("SET SESSION search_path='%s'" % schema_name)`
- **Exploit Scenario:** Attacker could manipulate SQL queries to access/modify/delete data
- **Fix Recommendation:** Use parameterized queries/prepared statements
- **Confidence:** Medium

### 2213. [High] SQL Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/dialects/sqlite/base.py:2234
- **Description:** SQL f-string interpolation
- **Evidence (Line):** `cursor.execute(f"PRAGMA read_uncommitted = {isolation_level}")`
- **Exploit Scenario:** Attacker could manipulate SQL queries to access/modify/delete data
- **Fix Recommendation:** Use parameterized queries/prepared statements
- **Confidence:** Medium

### 2214. [High] SQL Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/dialects/sqlite/provision.py:121
- **Description:** SQL f-string interpolation
- **Evidence (Line):** `dbapi_connection.execute(`
- **Exploit Scenario:** Attacker could manipulate SQL queries to access/modify/delete data
- **Fix Recommendation:** Use parameterized queries/prepared statements
- **Confidence:** Medium

### 2215. [High] SQL Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/dialects/sqlite/pysqlcipher.py:144
- **Description:** SQL f-string interpolation
- **Evidence (Line):** `cursor.execute(f"pragma key={passphrase}")`
- **Exploit Scenario:** Attacker could manipulate SQL queries to access/modify/delete data
- **Fix Recommendation:** Use parameterized queries/prepared statements
- **Confidence:** Medium

### 2216. [High] SQL Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/dialects/sqlite/pysqlcipher.py:146
- **Description:** SQL f-string interpolation
- **Evidence (Line):** `cursor.execute(f"pragma {prag}={value}")`
- **Exploit Scenario:** Attacker could manipulate SQL queries to access/modify/delete data
- **Fix Recommendation:** Use parameterized queries/prepared statements
- **Confidence:** Medium

### 2217. [High] SQL Injection
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/engine/base.py:3112
- **Description:** SQL % formatting
- **Evidence (Line):** `cursor.execute("use %s" % shards[shard_id])`
- **Exploit Scenario:** Attacker could manipulate SQL queries to access/modify/delete data
- **Fix Recommendation:** Use parameterized queries/prepared statements
- **Confidence:** Medium

### 2218. [High] SQL Injection
- **File:** .pytest_venv/share/jupyter/labextensions/jupyterlab-plotly/static/0.03957c7088ad5ed0.js:3526
- **Description:** SQL string concatenation
- **Evidence (Line):** ``),w=c.createShader(c.FRAGMENT_SHADER);if(c.isContextLost()){this.failedToCreate=!0;return}c.shaderS`
- **Exploit Scenario:** Attacker could manipulate SQL queries to access/modify/delete data
- **Fix Recommendation:** Use parameterized queries/prepared statements
- **Confidence:** Medium

### 2219. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/botocore/compat.py:146
- **Description:** MD5 usage
- **Evidence (Line):** `hashlib.md5(usedforsecurity=False)`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2220. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/botocore/compat.py:152
- **Description:** MD5 usage
- **Evidence (Line):** `def get_md5(*args, **kwargs):`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2221. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/botocore/compat.py:162
- **Description:** MD5 usage
- **Evidence (Line):** `return hashlib.md5(*args, **kwargs)`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2222. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/botocore/compat.py:146
- **Description:** Weak hash algorithm
- **Evidence (Line):** `hashlib.md5(usedforsecurity=False)`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2223. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/botocore/compat.py:162
- **Description:** Weak hash algorithm
- **Evidence (Line):** `return hashlib.md5(*args, **kwargs)`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2224. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/botocore/credentials.py:847
- **Description:** SHA1 usage
- **Evidence (Line):** `argument_hash = sha1(args.encode('utf-8')).hexdigest()`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2225. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/botocore/credentials.py:2299
- **Description:** SHA1 usage
- **Evidence (Line):** `argument_hash = sha1(args.encode('utf-8')).hexdigest()`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2226. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/botocore/handlers.py:329
- **Description:** MD5 usage
- **Evidence (Line):** `def sse_md5(params, **kwargs):`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2227. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/botocore/handlers.py:336
- **Description:** MD5 usage
- **Evidence (Line):** `_sse_md5(params, 'SSECustomer')`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2228. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/botocore/handlers.py:339
- **Description:** MD5 usage
- **Evidence (Line):** `def copy_source_sse_md5(params, **kwargs):`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2229. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/botocore/handlers.py:346
- **Description:** MD5 usage
- **Evidence (Line):** `_sse_md5(params, 'CopySourceSSECustomer')`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2230. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/botocore/handlers.py:349
- **Description:** MD5 usage
- **Evidence (Line):** `def _sse_md5(params, sse_member_prefix='SSECustomer'):`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2231. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/botocore/handlers.py:358
- **Description:** MD5 usage
- **Evidence (Line):** `md5_val = get_md5(key_as_bytes, usedforsecurity=False).digest()`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2232. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/botocore/httpchecksum.py:430
- **Description:** MD5 usage
- **Evidence (Line):** `conditionally_calculate_md5(request)`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2233. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/botocore/httpchecksum.py:169
- **Description:** SHA1 usage
- **Evidence (Line):** `self._checksum = sha1()`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2234. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/botocore/utils.py:3274
- **Description:** MD5 usage
- **Evidence (Line):** `def calculate_md5(body, **kwargs):`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2235. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/botocore/utils.py:3285
- **Description:** MD5 usage
- **Evidence (Line):** `md5 = get_md5(body_bytes, usedforsecurity=False)`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2236. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/botocore/utils.py:3292
- **Description:** MD5 usage
- **Evidence (Line):** `md5 = get_md5(usedforsecurity=False)`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2237. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/botocore/utils.py:3339
- **Description:** MD5 usage
- **Evidence (Line):** `conditionally_calculate_md5(params, **kwargs)`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2238. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/botocore/utils.py:3361
- **Description:** MD5 usage
- **Evidence (Line):** `def conditionally_calculate_md5(params, **kwargs):`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2239. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/botocore/utils.py:3382
- **Description:** MD5 usage
- **Evidence (Line):** `md5_digest = calculate_md5(body, **kwargs)`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2240. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/botocore/utils.py:3406
- **Description:** SHA1 usage
- **Evidence (Line):** `return hashlib.sha1(input_str.encode('utf-8')).hexdigest()`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2241. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/botocore/utils.py:3406
- **Description:** Weak hash algorithm
- **Evidence (Line):** `return hashlib.sha1(input_str.encode('utf-8')).hexdigest()`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2242. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/faker/providers/misc/__init__.py:72
- **Description:** MD5 usage
- **Evidence (Line):** `def md5(self) -> str: ...`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2243. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/faker/providers/misc/__init__.py:75
- **Description:** MD5 usage
- **Evidence (Line):** `def md5(self, raw_output: Literal[True]) -> bytes: ...`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2244. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/faker/providers/misc/__init__.py:78
- **Description:** MD5 usage
- **Evidence (Line):** `def md5(self, raw_output: Literal[False]) -> str: ...`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2245. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/faker/providers/misc/__init__.py:80
- **Description:** MD5 usage
- **Evidence (Line):** `def md5(self, raw_output: bool = False) -> Union[bytes, str]:`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2246. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/faker/providers/misc/__init__.py:89
- **Description:** MD5 usage
- **Evidence (Line):** `res: hashlib._Hash = hashlib.md5(str(self.generator.random.random()).encode())`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2247. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/faker/providers/misc/__init__.py:95
- **Description:** SHA1 usage
- **Evidence (Line):** `def sha1(self) -> str: ...`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2248. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/faker/providers/misc/__init__.py:98
- **Description:** SHA1 usage
- **Evidence (Line):** `def sha1(self, raw_output: Literal[True]) -> bytes: ...`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2249. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/faker/providers/misc/__init__.py:101
- **Description:** SHA1 usage
- **Evidence (Line):** `def sha1(self, raw_output: Literal[False]) -> str: ...`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2250. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/faker/providers/misc/__init__.py:103
- **Description:** SHA1 usage
- **Evidence (Line):** `def sha1(self, raw_output: bool = False) -> Union[bytes, str]:`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2251. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/faker/providers/misc/__init__.py:112
- **Description:** SHA1 usage
- **Evidence (Line):** `res: hashlib._Hash = hashlib.sha1(str(self.generator.random.random()).encode())`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2252. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/faker/providers/misc/__init__.py:89
- **Description:** Weak hash algorithm
- **Evidence (Line):** `res: hashlib._Hash = hashlib.md5(str(self.generator.random.random()).encode())`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2253. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/faker/providers/misc/__init__.py:112
- **Description:** Weak hash algorithm
- **Evidence (Line):** `res: hashlib._Hash = hashlib.sha1(str(self.generator.random.random()).encode())`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2254. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/fsspec/tests/abstract/__init__.py:228
- **Description:** MD5 usage
- **Evidence (Line):** `hashed_i = md5(str(i).encode("utf-8")).hexdigest()`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2255. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/fsspec/tests/abstract/copy.py:548
- **Description:** MD5 usage
- **Evidence (Line):** `hashed_i = md5(str(i).encode("utf-8")).hexdigest()`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2256. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/fsspec/tests/abstract/get.py:576
- **Description:** MD5 usage
- **Evidence (Line):** `hashed_i = md5(str(i).encode("utf-8")).hexdigest()`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2257. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/fsspec/tests/abstract/put.py:582
- **Description:** MD5 usage
- **Evidence (Line):** `hashed_i = md5(str(i).encode("utf-8")).hexdigest()`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2258. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/fsspec/utils.py:321
- **Description:** MD5 usage
- **Evidence (Line):** `h = md5(str(args).encode())`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2259. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/fsspec/utils.py:324
- **Description:** MD5 usage
- **Evidence (Line):** `h = md5(str(args).encode(), usedforsecurity=False)`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2260. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/jinja2/bccache.py:156
- **Description:** SHA1 usage
- **Evidence (Line):** `hash = sha1(name.encode("utf-8"))`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2261. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/jinja2/bccache.py:165
- **Description:** SHA1 usage
- **Evidence (Line):** `return sha1(source.encode("utf-8")).hexdigest()`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2262. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/jinja2/loaders.py:661
- **Description:** SHA1 usage
- **Evidence (Line):** `return "tmpl_" + sha1(name.encode("utf-8")).hexdigest()`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2263. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/test/test_hashing.py:212
- **Description:** MD5 usage
- **Evidence (Line):** `In [23]: %timeit hashlib.md5(a).hexdigest()`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2264. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/test/test_hashing.py:215
- **Description:** MD5 usage
- **Evidence (Line):** `In [24]: %timeit hashlib.md5(pickle.dumps(a, protocol=2)).hexdigest()`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2265. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/test/test_hashing.py:218
- **Description:** MD5 usage
- **Evidence (Line):** `In [25]: %timeit hashlib.md5(cPickle.dumps(a, protocol=2)).hexdigest()`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2266. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/test/test_hashing.py:228
- **Description:** MD5 usage
- **Evidence (Line):** `return hashlib.md5(memoryview(x)).hexdigest()`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2267. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/test/test_hashing.py:212
- **Description:** Weak hash algorithm
- **Evidence (Line):** `In [23]: %timeit hashlib.md5(a).hexdigest()`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2268. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/test/test_hashing.py:215
- **Description:** Weak hash algorithm
- **Evidence (Line):** `In [24]: %timeit hashlib.md5(pickle.dumps(a, protocol=2)).hexdigest()`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2269. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/test/test_hashing.py:218
- **Description:** Weak hash algorithm
- **Evidence (Line):** `In [25]: %timeit hashlib.md5(cPickle.dumps(a, protocol=2)).hexdigest()`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2270. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/joblib/test/test_hashing.py:228
- **Description:** Weak hash algorithm
- **Evidence (Line):** `return hashlib.md5(memoryview(x)).hexdigest()`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2271. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/distutils/fcompiler/gnu.py:402
- **Description:** SHA1 usage
- **Evidence (Line):** `h = hashlib.sha1()`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2272. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/numpy/distutils/fcompiler/gnu.py:402
- **Description:** Weak hash algorithm
- **Evidence (Line):** `h = hashlib.sha1()`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2273. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/pandas/tests/io/pytables/test_store.py:61
- **Description:** Weak hash algorithm
- **Evidence (Line):** `def checksum(filename, hash_factory=hashlib.md5, chunk_num_blocks=128):`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2274. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/pip/_vendor/requests/auth.py:179
- **Description:** MD5 usage
- **Evidence (Line):** `return hashlib.md5(x, usedforsecurity=False).hexdigest()`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2275. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/pip/_vendor/requests/auth.py:187
- **Description:** SHA1 usage
- **Evidence (Line):** `return hashlib.sha1(x, usedforsecurity=False).hexdigest()`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2276. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/pip/_vendor/requests/auth.py:237
- **Description:** SHA1 usage
- **Evidence (Line):** `cnonce = hashlib.sha1(s, usedforsecurity=False).hexdigest()[:16]`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2277. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/pip/_vendor/requests/auth.py:179
- **Description:** Weak hash algorithm
- **Evidence (Line):** `return hashlib.md5(x, usedforsecurity=False).hexdigest()`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2278. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/pip/_vendor/requests/auth.py:187
- **Description:** Weak hash algorithm
- **Evidence (Line):** `return hashlib.sha1(x, usedforsecurity=False).hexdigest()`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2279. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/pip/_vendor/requests/auth.py:237
- **Description:** Weak hash algorithm
- **Evidence (Line):** `cnonce = hashlib.sha1(s, usedforsecurity=False).hexdigest()[:16]`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2280. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/pip/_vendor/rich/tree.py:18
- **Description:** DES usage
- **Evidence (Line):** `ASCII_GUIDES (GuideType): Guide lines used when Console.ascii_only is True.`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2281. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/pip/_vendor/rich/tree.py:19
- **Description:** DES usage
- **Evidence (Line):** `TREE_GUIDES (List[GuideType, GuideType, GuideType]): Default guide lines.`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2282. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/sklearn/datasets/_openml.py:548
- **Description:** MD5 usage
- **Evidence (Line):** `md5 = hashlib.md5()`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2283. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/sklearn/datasets/_openml.py:548
- **Description:** Weak hash algorithm
- **Evidence (Line):** `md5 = hashlib.md5()`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2284. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/dialects/mysql/base.py:497
- **Description:** MD5 usage
- **Evidence (Line):** `table.select(table.c.password == func.md5("plaintext"))`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2285. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/util/compat.py:216
- **Description:** MD5 usage
- **Evidence (Line):** `return hashlib.md5(usedforsecurity=False)`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2286. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/util/compat.py:221
- **Description:** MD5 usage
- **Evidence (Line):** `return hashlib.md5()`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2287. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/util/compat.py:216
- **Description:** Weak hash algorithm
- **Evidence (Line):** `return hashlib.md5(usedforsecurity=False)`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2288. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/sqlalchemy/util/compat.py:221
- **Description:** Weak hash algorithm
- **Evidence (Line):** `return hashlib.md5()`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2289. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_logging/_internal.py:1597
- **Description:** MD5 usage
- **Evidence (Line):** `h = hashlib.md5(usedforsecurity=False)`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2290. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/_logging/_internal.py:1597
- **Description:** Weak hash algorithm
- **Evidence (Line):** `h = hashlib.md5(usedforsecurity=False)`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2291. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/distributed/distributed_c10d.py:6497
- **Description:** SHA1 usage
- **Evidence (Line):** `return hashlib.sha1(bytes(unique_str, "utf-8"), usedforsecurity=False).hexdigest()`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2292. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/distributed/distributed_c10d.py:6497
- **Description:** Weak hash algorithm
- **Evidence (Line):** `return hashlib.sha1(bytes(unique_str, "utf-8"), usedforsecurity=False).hexdigest()`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2293. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/fx/passes/graph_drawer.py:174
- **Description:** MD5 usage
- **Evidence (Line):** `hashlib.md5(`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2294. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/fx/passes/graph_drawer.py:174
- **Description:** Weak hash algorithm
- **Evidence (Line):** `hashlib.md5(`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2295. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/utils/_config_module.py:834
- **Description:** MD5 usage
- **Evidence (Line):** `hashlib.md5(`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2296. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/utils/_config_module.py:834
- **Description:** Weak hash algorithm
- **Evidence (Line):** `hashlib.md5(`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2297. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/utils/_content_store.py:106
- **Description:** SHA1 usage
- **Evidence (Line):** `sha1 = hashlib.sha1(usedforsecurity=False)`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2298. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/utils/_content_store.py:106
- **Description:** Weak hash algorithm
- **Evidence (Line):** `sha1 = hashlib.sha1(usedforsecurity=False)`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2299. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/utils/hipify/hipify_python.py:702
- **Description:** MD5 usage
- **Evidence (Line):** `self._hash = hashlib.md5(usedforsecurity=False)`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2300. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/torch/utils/hipify/hipify_python.py:702
- **Description:** Weak hash algorithm
- **Evidence (Line):** `self._hash = hashlib.md5(usedforsecurity=False)`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2301. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/torchgen/utils.py:108
- **Description:** SHA1 usage
- **Evidence (Line):** `sha1 = hashlib.sha1(s.encode("latin1"), usedforsecurity=False).digest()`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High

### 2302. [Medium] Weak Cryptography
- **File:** .pytest_venv/lib/python3.11/site-packages/torchgen/utils.py:108
- **Description:** Weak hash algorithm
- **Evidence (Line):** `sha1 = hashlib.sha1(s.encode("latin1"), usedforsecurity=False).digest()`
- **Exploit Scenario:** Weak algorithms vulnerable to collision/brute force attacks
- **Fix Recommendation:** Use modern algorithms (SHA-256+, bcrypt/argon2 for passwords, secrets module for tokens)
- **Confidence:** High
