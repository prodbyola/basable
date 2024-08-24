import * as React from 'react';
import Container from '@mui/material/Container';
import useStyles from '../../styles/styles.js';
import Grid from '@mui/material/Grid';
import Box from '@mui/material/Box';
import Toolbar from '@mui/material/Toolbar';
import Total1 from '../../assets/images/Total1.png';
import Total2 from '../../assets/images/Total2.png';
import Total3 from '../../assets/images/Total3.png';
import Total4 from '../../assets/images/Total4.png';
import Frame1 from '../../assets/images/Frame1.png';
import Frame2 from '../../assets/images/Frame2.png';
import CardMedia from '@mui/material/CardMedia';
import Card from '@mui/material/Card';

const headerHeight = 80;

interface totalCard {
  id: number;
  imagePath: string;
}
interface frameCard {
  imagePath: string;
  frameClass: string;
}
const totalCards: totalCard[] = [
  { id: 1, imagePath: Total1 },
  { id: 2, imagePath: Total2 },
  { id: 3, imagePath: Total3 },
  { id: 4, imagePath: Total4 }
];

function DashboardMain() {
  const classes = useStyles();
  const FrameCards: frameCard[] = [
    { imagePath: Frame1, frameClass: classes.cardFrame1 },
    { imagePath: Frame2, frameClass: classes.cardFrame2 }
  ];

  return (
    <Box sx={{ width: '100%' }}>
      <Toolbar sx={{ height: headerHeight }} />
      <Box className={classes.mainContainer}>
        <Container className={classes.imageGrid}>
          <Grid container spacing={2}>
            {totalCards.map((totalCard) => (
              <Grid item key={totalCard.id} md={3} sm={6} xs={12}>
                <Card className={classes.card}>
                  <CardMedia
                    className={classes.cardMedia}
                    image={totalCard.imagePath}
                  />
                </Card>
              </Grid>
            ))}
            {FrameCards.map((frameCard) => (
              <Grid item key={5} xs={12} sm={12} md={12}>
                <Card className={classes.card}>
                  <CardMedia
                    className={frameCard.frameClass}
                    image={frameCard.imagePath}
                  />
                </Card>
              </Grid>
            ))}
          </Grid>
        </Container>
      </Box>
    </Box>
  );
}

export default DashboardMain;
