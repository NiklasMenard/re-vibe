import styled from 'styled-components';
import { useState } from 'react';
import { FlexColumn } from '../styles/layouts';
import { useAuth } from '../hooks/useAuth';

const LoginPage = () => {
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');

  const { login, error } = useAuth();

  const handleSubmit = (event: React.FormEvent) => {
    event.preventDefault();
    login(email, password);
  };

  return (
    <FlexColumn>
      <LoginContainer>
        <h2>Login BOMBO</h2>
        <div className="error">{error}</div>
        <LoginForm onSubmit={handleSubmit}>
          <div className="form-group">
            <label htmlFor="email">Username:</label>
            <input
              type="text"
              id="email"
              value={email}
              onChange={(e) => setEmail(e.target.value)}
              required
            />
          </div>
          <div className="form-group">
            <label htmlFor="password">Password:</label>
            <input
              type="password"
              id="password"
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              required
            />
          </div>

          <button type="submit">Login</button>
        </LoginForm>
      </LoginContainer>
    </FlexColumn>
  );
};

export default LoginPage;

const LoginContainer = styled.div`
  display: flex;
  justify-content: center;
  flex-direction: column;
  height: 20rem;
  width: 20rem;
  margin: auto;

  .error {
    color: red;
    height: 1.5rem;
  }

  .form-group {
    margin-top: 1rem;
  }

  input {
    width: 100%;
    padding: 8px;
    box-sizing: border-box;
  }

  label {
    display: block;
    margin-bottom: 5px;
  }

  input {
    width: 100%;
    padding: 8px;
    box-sizing: border-box;
  }

  button {
    padding: 10px 15px;
    background-color: #007bff;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    margin-top: 1rem;
  }

  button:hover {
    background-color: #0056b3;
  }
`;

const LoginForm = styled.form``;
