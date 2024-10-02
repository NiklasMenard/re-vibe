import { useState } from 'react';
import { useAuth } from '../hooks/useAuth';

import { Button } from '@/components/Buttons';
import { Input } from '@/components/Input';

import Header from '@/components/Header';
import { Label } from '@radix-ui/react-label';

const LoginPage = () => {
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');

  const { login, error } = useAuth();

  const handleSubmit = (event: React.FormEvent) => {
    event.preventDefault();
    login(email, password);
  };

  return (
    <div className="flex flex-col items-center justify-center h-screen">
      <Header />
      <div className="flex flex-col items-center p-6 bg-white shadow-md rounded-lg max-w-sm mx-10">
        <h2 className="text-xl font-bold mb-4">Login</h2>
        {error && <div className="text-red-500 mb-4">{error}</div>}
        <form onSubmit={handleSubmit} className="w-full">
          <div className="mb-10">
            <p>Username: test-user </p>
            <p className="mb-10">Password: password</p>
            <Label htmlFor="email">Username:</Label>
            <Input
              type="text"
              id="email"
              value={email}
              onChange={(e) => setEmail(e.target.value)}
              required
              className="w-full p-2 borde"
            />
          </div>
          <div className="mb-10">
            <Label htmlFor="password">Password:</Label>

            <Input
              type="password"
              id="password"
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              required
              className="w-full p-2 border"
            />
          </div>

          <Button type="submit" className="w-full">
            Login
          </Button>
        </form>
      </div>
    </div>
  );
};

export default LoginPage;
