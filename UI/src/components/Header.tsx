import styled from 'styled-components';
import Logo from './Logo';

const Header = () => {
  return (
    <HeaderWrapper>
      <Logo />
    </HeaderWrapper>
  );
};

const HeaderWrapper = styled.header`
  display: flex;
  background-color: #ffff;
  padding: 0 1.5rem;
  text-align: center;
  font-size: 1.5rem;
  height: 5vh;
  border-bottom: 1px solid ${({ theme }) => theme.colors.jet};
`;

export default Header;
