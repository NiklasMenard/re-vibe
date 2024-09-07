import styled from 'styled-components';
import PrimaryButton from '../components/Buttons';
import { useNavigate } from 'react-router-dom';

import Header from '../components/Header';

const Home = () => {
  const navigate = useNavigate();

  return (
    <HomeLayout>
      <Header />
      <ContentGrid>
        <HeroText>
          <h1>Medium length hero heading goes here</h1>
          <p>
            Lorem ipsum dolor sit amet, consectetur adipiscing elit. Suspendisse varius enim in eros
            elementum tristique. Duis cursus, mi quis viverra ornare, eros dolor interdum nulla, ut
            commodo diam libero vitae erat.
          </p>
          <PrimaryButton onClick={() => navigate('/products')}>Explore</PrimaryButton>
        </HeroText>
        <HeroImage>pictures</HeroImage>

        <FeatureText>
          <h2>Discover the Easiest Way to Buy and Sell Products Online</h2>
          <p>
            Our platform offers a seamless and secure experience for buying and selling a wide
            variety of products. With intuitive navigation and secure transactions, you can trust us
            to provide a user-friendly marketplace for all your needs.
          </p>
        </FeatureText>
        <FeatureImage>pictures</FeatureImage>
      </ContentGrid>

      <Footer>Footer</Footer>
    </HomeLayout>
  );
};

const HomeLayout = styled.div`
  display: flex;
  flex-direction: column;
`;

const ContentGrid = styled.div`
  display: grid;
  grid-template-columns: 1fr;
  grid-template-rows: repeat(4, 1fr);
  height: 90vh;

  div {
    border-bottom: 1px solid ${({ theme }) => theme.colors.jet};
  }
  @media ${({ theme }) => theme.devices.md} {
    grid-template-columns: repeat(2, 1fr);
    grid-template-rows: repeat(2, 1fr);
  }
`;

const HeroText = styled.div`
  display: flex;
  flex-direction: column;
  justify-content: center;
  padding: 2.5rem;
  background-color: ${({ theme }) => theme.colors.vanilla};
`;

const HeroImage = styled.div`
  background-color: lightgray;
`;

const FeatureText = styled.div`
  display: flex;
  flex-direction: column;
  justify-content: center;
  padding: 2.5rem;
  background-color: ${({ theme }) => theme.colors.coral};
`;

const FeatureImage = styled.div`
  background-color: lightgray;
`;

const Footer = styled.footer`
  display: flex;
  height: 5vh;
  background-color: ${({ theme }) => theme.colors.viridian};
`;

export default Home;
