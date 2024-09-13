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
  position: fixed;
  top: 0;
  width: 100%;
  z-index: 1;
  background: #fff;
  border-bottom: 1px solid #ccc;
  padding: 1rem;
  height: 7rem;
`;

export default Header;
