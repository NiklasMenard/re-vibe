import styled from 'styled-components';

const Footer = () => {
  return <FooterWrapper>Footer</FooterWrapper>;
};

const FooterWrapper = styled.footer`
  position: fixed;
  bottom: 0;
  width: 100%;
  z-index: 1;
  background-color: ${({ theme }) => theme.colors.viridian};
  border-top: 1px solid #ccc;
  height: 8rem;
`;

export default Footer;
