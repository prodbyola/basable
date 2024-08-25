import { makeStyles } from '@material-ui/core/styles';

const useStyles = makeStyles((theme) => ({
  // ------Header--------
  toolbar: {
    margin: '8px',
    display: 'flex',
    justifyContent: 'space-between'
  },
  searchbar: {
    borderRadius: '10px',
    marginLeft: '100px'
  },
  headerright: {
    display: 'flex',
    justifyContent: 'flex-end',
    alignItems: 'center'
  },
  typography: {
    textTransform: 'none'
  },
  // --------Navbar
  database: {
    paddingLeft: '0px'
  },
  // ----Dashboard Main-----
  mainContainer: {
    display: 'flex',
    justifyContent: 'center',
    padding: '30px',
    height: 'calc(100% - headerWidth)'
  },
  imageGrid: {
    padding: '20px 0',
    margin: '0px',
  },
  card: {
    height: '100%',
    display: 'flex',
    flexDirection: 'column'
  },
  cardMedia: {
    paddingTop: '56.25%'
  },
  cardFrame1: {
    paddingTop: '27%'
  },
  cardFrame2: {
    paddingTop: '20%'
  }
}));

export default useStyles;
