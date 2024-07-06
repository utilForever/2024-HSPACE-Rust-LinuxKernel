#include <iostream>

class SpreadSheet
{
public:
    SpreadSheet(int id, int rows, int cols)
        : m_id(id), m_rows(rows), m_cols(cols)
    {
        m_data = new int *[rows];

        for (int i = 0; i < rows; ++i)
        {
            m_data[i] = new int[cols];
        }
    }

    SpreadSheet(const SpreadSheet &src)
        : SpreadSheet(src.m_id, src.m_rows, src.m_cols)
    {
        for (int i = 0; i < m_rows; ++i)
        {
            for (int j = 0; j < m_cols; ++j)
            {
                m_data[i][j] = src.m_data[i][j];
            }
        }
    }

    ~SpreadSheet()
    {
        for (int i = 0; i < m_rows; ++i)
        {
            delete[] m_data[i];
        }

        delete[] m_data;
    }

    void SetCell(int row, int col, int value)
    {
        m_data[row][col] = value;
    }

    int GetCell(int row, int col) const
    {
        return m_data[row][col];
    }

private:
    int m_id;
    int m_rows;
    int m_cols;
    int **m_data;
};

int main()
{
    SpreadSheet sheet1(1, 10, 5);

    {
        SpreadSheet sheet2(sheet1);
        sheet1.SetCell(0, 0, 42);

        // sheet2.GetCell(0, 0) == 42
        std::cout << sheet2.GetCell(0, 0) << '\n';
    }

    // sheet1.GetCell(0, 0) == 42
    std::cout << sheet1.GetCell(0, 0) << '\n';

    return 0;
}
