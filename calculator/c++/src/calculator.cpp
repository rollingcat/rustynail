
#include <stdio.h>
#include <iostream>
#include <sstream>
#include <stack>
#include <string>
#include <vector>
#include <cstdlib>
#include <algorithm>
#include <assert.h>

#define MAX_DIGIT_NUM 10

using namespace std;

vector<string> tokens;

void Tokenize(const string& input) {
    tokens.clear();

    stringstream iss(input);
    string line;

    size_t pos = 0;
    size_t prev = 0;
    while (getline(iss, line)) {
        while ((pos = line.find_first_of(" +-*()", prev)) != string::npos) {

            if (pos >= prev) {
                if (pos != prev)
                    tokens.push_back(line.substr(prev, pos - prev));
                if (line[pos] != ' ') {
                    char op[2] = { line[pos], 0 };
                    tokens.push_back(string(op));
                }
                prev = pos + 1;
            }
        }
        if (prev < line.length())
            tokens.push_back(line.substr(prev, string::npos));
    }
}

string ConvertInfixToPostfix() {
    stringstream postfix;
    vector<int> stack;
    string op("+-*");

    for (size_t i = 0; i < tokens.size(); ++i) {
        const string& token = tokens[i];

        size_t idx = op.find_first_of(token, 0);

        if (idx != -1 && token.length() == 1) {
            if (stack.empty()) {
                stack.push_back(idx);
            } else {
                while (!stack.empty()) {
                    int prec2 = stack.back() / 2;
                    int prec1 = idx / 2;
                    if (prec2 > prec1) {
                        postfix << op[stack.back()] << " ";
                        stack.pop_back();
                    } else {
                        break;
                    }
                }
                stack.push_back(idx);
            }
        } else if (token == "(") {
            stack.push_back(-2);
        } else if (token == ")") {
            while (stack.back() != -2) {
                postfix << op[stack.back()] << " ";
                stack.pop_back();
            }
            stack.pop_back();
        } else {
            int number = 0;
            if (istringstream(token) >> number) {
                postfix << token << " ";
            } else {
                cout << "Invalid operator or number!" << endl;
                return string();
            }
        }
    }
    while (!stack.empty()) {
        if (stack.back() == -2) {
            cout << "Parenthesis were mismatched!" << endl;
            return string();
        }
        postfix << op[stack.back()] << " ";
        stack.pop_back();
    }
    return postfix.str();
}

enum Comparing {
    Less,
    Greater,
    Equal
};

Comparing Compare(const string& a, const string& b) {
    if (a.length() > b.length())
        return Greater;
    else if (a.length() < b.length())
        return Less;

    for (size_t i = 0; i < a.length(); ++i) {
        if (a[i] > b[i])
            return Greater;
        else if (a[i] < b[i])
            return Less;
    }
    return Equal;
}

string Add(const string& a, const string& b)
{
    int firstNum, secondNum;
    istringstream(a) >> firstNum;
    istringstream(b) >> secondNum;
    int newNum = firstNum + secondNum;

    stringstream ss;
    ss << newNum;
    return ss.str();
}

string AddBigInt(const string& aa, const string& bb) {
    string a = aa;
    std::reverse(a.begin(), a.end());
    string b = bb;
    std::reverse(b.begin(), b.end());

    string result;
    size_t aLen = a.length();
    size_t bLen = b.length();
    size_t addLen = (aLen >= bLen) ? aLen : bLen;

    char transfer = 0;
    for (size_t i = 0; i < addLen; ++i) {
        char aChar, bChar;
        aChar = (i >= aLen) ? 0 : a[i] - '0';
        bChar = (i >= bLen) ? 0 : b[i] - '0';

        char sum = aChar + bChar + transfer;
        transfer = sum / 10;
        char num = sum % 10;
        result.insert(0, 1, num + '0');
    }
    if (transfer != 0) {
        result.insert(0, 1, transfer + '0');
    }
    return result;
}

string Subtract(const string& a, const string& b) {
    int firstNum, secondNum;
    istringstream(a) >> firstNum;
    istringstream(b) >> secondNum;
    int newNum = firstNum - secondNum;

    stringstream ss;
    ss << newNum;
    return ss.str();
}

string SubtractBigInt(const string& aa, const string& bb) {
    string a, b;

    Comparing comp = Compare(aa, bb);
    if (comp == Greater) {
        a = aa;
        b = bb;
    } else if (comp == Less) {
        a = bb;
        b = aa;
    } else
        return string("0");

    std::reverse(a.begin(), a.end());
    std::reverse(b.begin(), b.end());

    string result;
    size_t aLen = a.length();
    size_t bLen = b.length();
    size_t addLen = (aLen >= bLen) ? aLen : bLen;

    char borrow = 0;
    for (size_t i = 0; i < addLen; ++i) {
        char aChar, bChar;
        aChar = (i >= aLen) ? 0 : a[i] - '0';
        bChar = (i >= bLen) ? 0 : b[i] - '0';

        int sub = aChar - bChar - borrow;
        if (sub < 0) {
            borrow = 1;
            sub += 10;
        } else
            borrow = 0;

        char ch = sub + '0';
        result.append(&ch, 1);
    }

    while (result.back() == '0')
        result.pop_back();

    std::reverse(result.begin(), result.end());

    if (comp == Less)
        result.insert(0, 1, '-');
    return result;
}

string Multiply(const string& a, const string& b) {
    unsigned int firstNum, secondNum;
    istringstream(a) >> firstNum;
    istringstream(b) >> secondNum;
    unsigned int newNum = firstNum * secondNum;

    stringstream ss;
    ss << newNum;
    return ss.str();
}

void Normalize(vector<int>& input) {
    input.push_back(0);

    for (size_t i = 0; i < input.size(); ++i) {
        if (input[i] == 0) {
            continue;
        }
        input[i + 1] += input[i] / 10;
        input[i] %= 10;
    }
    while (input.back() == 0)
        input.pop_back();
}

string MultplyBigIntBruteForce(const string& aa, const string& bb) {
    string a = aa;
    string b = bb;
    reverse(a.begin(), a.end());
    reverse(b.begin(), b.end());

    bool aMinus = a.back() == '-';
    bool bMinus = b.back() == '-';
    if (aMinus) a.pop_back();
    if (bMinus) b.pop_back();

    bool minus = false;
    if (aMinus != bMinus)
        minus = true;

    vector<int> result(a.length() + b.length() + 1, 0);
    for (size_t i = 0; i < a.length(); ++i) {
        for (size_t j = 0; j < b.length(); ++j) {
            result[i+j] += (a[i] - '0') * (b[j] - '0');
        }
    }
    Normalize(result);

    string ret;
    ret.reserve(result.size() + 1);
    if (minus)
        ret.append('-', 1);

    char ch = 0;
    for (vector<int>::iterator iter = result.end() - 1; iter != result.begin(); --iter) {
        ch = *iter + '0';
        ret.append(&ch, 1);
    }
    ch = *result.begin() + '0';
    ret.append(&ch, 1);
    return ret;
}

string MultiplyBigInt(const string& a, const string& b) {
    return MultplyBigIntBruteForce(a, b);
}

bool RPNWithBigIntegers(const string& input, string& result) {
    istringstream iss(input);
    vector<string> stack;
    string token;
    while (iss >> token) {
        if (token.length() == 1 && !std::isdigit(token[0])) {
            size_t stackSize = stack.size();
            if (stackSize < 2) {
                cout << "Invalid postfix notation!" << endl;
                return false;
            }
            string& secondNum = stack[stackSize - 1];
            string& firstNum = stack[stackSize - 2];

            int secondLen = secondNum.length();
            int firstLen = firstNum.length();

            string newNum;
            if (token == "+") {
                if (firstLen >= (MAX_DIGIT_NUM - 1) || secondLen >= (MAX_DIGIT_NUM - 1)) {
                    newNum = AddBigInt(firstNum, secondNum);
                } else {
                    newNum = Add(firstNum, secondNum);
                }
            } else if (token == "-") {
                if (firstLen >= (MAX_DIGIT_NUM - 1) || secondLen >= (MAX_DIGIT_NUM - 1)) {
                    newNum = SubtractBigInt(firstNum, secondNum);
                } else {
                    newNum = Subtract(firstNum, secondNum);
                }
            } else if (token == "*") {
                if ((firstLen + secondLen) > MAX_DIGIT_NUM) {
                    newNum = MultiplyBigInt(firstNum, secondNum);
                } else {
                    newNum = Multiply(firstNum, secondNum);
                }
            } else {
                cout << "Unsupported operator: " << token << endl;
                exit(0);
            }
            stack.pop_back();
            stack.pop_back();
            stack.push_back(newNum);
        } else {
            stack.push_back(token);
        }
    }
    if (stack.size() != 1) {
        cout << "Invalid postfix notation!" << endl;
        return false;
    }
    result = stack.back();
    return true;
}

bool RPN(const string& input, int& result) {
    istringstream iss(input);
    vector<int> stack;
    string token;
    while (iss >> token) {
        int number = 0;
        if (istringstream(token) >> number) {
            stack.push_back(number);
        } else {
            if (stack.size() < 2) {
                cout << "Invalid postfix notation!" << endl;
                return false;
            }
            int secondNum = stack.back();
            stack.pop_back();
            int firstNum = stack.back();
            stack.pop_back();

            int newNum = 0;
            if (token == "+") {
                newNum = firstNum + secondNum;
            } else if (token == "-") {
                newNum = firstNum - secondNum;
            } else if (token == "*") {
                newNum = firstNum * secondNum;
            } else {
                cout << "Unsupported operator: " << token << endl;
                exit(0);
            }

            stack.push_back(newNum);
        }
    }
    if (stack.size() != 1) {
        cout << "Invalid postfix notation!" << endl;
        return false;
    }
    result = stack.back();
    return true;
}

int main() {
    cout << "<Calculator>" << endl;

    string ret1 = Add("111111111", "9");
    string ret2 = AddBigInt("111111111", "9");
    assert(ret1 == ret2);

    ret1 = Subtract("123456789", "987654321");
    ret2 = SubtractBigInt("123456789", "987654321");
    assert(ret1 == ret2);

    ret1 = Multiply("12345", "65432");
    ret2 = MultiplyBigInt("12345", "65432");
    assert(ret1 == ret2);

    const char *tests[] = {
        "3 + 4 * 2 - ( 1 - 5 ) + 7",    /* RC mandated: OK */
        "123",                          /* OK */
        "3+4 * 2 * ( 5 - 3 )",          /* OK */
        "(((((((1+2+3*(4 + 5))))))",    /* bad parens */
        "a - (b + c+d * 4)!",           /* unknown op */
        "3 + 4 * 2 ( 1 - 5 ) + 7",      /* invalid */
        "3 + 4 * 2 - * ( 1 - 5 ) + 7",  /* invalid */
        "(1*2)*3 + (10 - 5) *2",        /* OK */
        0
    };

    for (int i = 0; tests[i]; ++i) {
        printf("Testing string '%s'\n", tests[i]);

        Tokenize(string(tests[i]));
        string postfix = ConvertInfixToPostfix();

        if (postfix.empty())
            continue;

        cout << "postfix: " << postfix << endl;

        string result;
        if (RPNWithBigIntegers(postfix, result)) {
            cout << "result: " << result << endl;
        }
    }

	return 0;
}
