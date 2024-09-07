import React from 'react';
import styled from 'styled-components';

interface PrimaryButtonProps {
  onClick?: () => void;
  className?: string;
  children?: React.ReactNode;
}

const PrimaryButton: React.FC<PrimaryButtonProps> = ({ children, onClick, className }) => {
  return (
    <Button className={className} onClick={onClick}>
      {children}
    </Button>
  );
};

const Button = styled.button`
  width: 15rem;
  height: 3rem;
  display: flex;
  align-items: center;
  justify-content: center;
  background-color: ${({ theme }) => theme.colors.jet};
  color: white;
  border: none;
  cursor: pointer;
  font-size: 1rem;
  font-family: sans-serif;

  transition: all 0.3s ease;

  &:hover {
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
    transform: scale(1.02);
  }

  &:focus {
    outline: none;
  }
`;

export default PrimaryButton;
